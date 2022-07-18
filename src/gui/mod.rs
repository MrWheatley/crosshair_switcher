pub mod crosshair_list;
mod weapon_list;

use crate::{ExplosionEffect, WeaponFile, USES_EXPLOSION};
use crosshair_list::CrosshairList;
use weapon_list::WeaponList;

use std::path::Path;

use anyhow::{anyhow, bail, Result};
use fltk::{app, button, enums, group::Flex, menu, prelude::*, text, window};

#[derive(Clone, Copy)]
pub enum LogType {
    Info,
    Error,
}

#[derive(Clone)]
pub struct Log(text::TextDisplay);

impl Log {
    fn new() -> Self {
        let mut log = text::TextDisplay::default_fill();
        log.set_text_font(enums::Font::Courier);
        log.set_buffer(text::TextBuffer::default());
        log.set_scrollbar_align(enums::Align::Right);

        Self(log)
    }

    pub fn log(&mut self, log_type: LogType, msg: impl std::fmt::Display) {
        let mut buffer = self.0.buffer().unwrap();

        buffer.append(&format!(
            "[{}] {}\n",
            match log_type {
                LogType::Info => "Info",
                LogType::Error => "Error",
            },
            msg
        ));

        self.0
            .scroll(self.0.count_lines(0, buffer.length(), true), 0);
    }
}

macro_rules! error_log {
    ($log:expr, $fun:expr) => {
        if let Err(e) = $fun {
            $log.log(LogType::Error, e)
        }
    };
}

#[derive(Clone, Copy)]
pub enum ButtonMsg {
    Apply,
    ToClass,
    ToSlot,
    ToAll,
}

#[derive(Clone, Copy)]
pub enum Message {
    WeaponListClicked,
    ButtonClicked(ButtonMsg),
    CrosshairRadioClicked,
    ExplosionRadioClicked,
    Redraw,
}

pub struct App {
    app: app::App,

    weapon_list: WeaponList,
    info: text::TextDisplay,
    crosshair_radio: button::RadioRoundButton,
    explosion_input: menu::Choice,
    button_group: (button::Button, button::Button, button::Button),
    crosshair_list: CrosshairList,
    log: Log,

    s: app::Sender<Message>,
    r: app::Receiver<Message>,
}

impl App {
    pub fn new(title: &str) -> Self {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
        let theme = fltk_theme::ColorTheme::new(fltk_theme::color_themes::BLACK_THEME);
        theme.apply();

        let mut wind = window::Window::default()
            .with_size(900, 800)
            .with_label(title)
            .center_screen();
        wind.size_range(900, 600, 0, 0);
        wind.make_resizable(true);

        let (s, r) = app::channel();

        let mut main_column = Flex::default_fill().column();
        main_column.set_margin(5);

        let (weapon_list, info, crosshair_radio, explosion_input, button_group, crosshair_list) = {
            let row = Flex::default_fill().row();

            let weapon_list = WeaponList::new(s);

            let mut col = Flex::default_fill().column();

            let mut info = text::TextDisplay::default_fill();
            info.set_text_font(enums::Font::Courier);
            info.set_scrollbar_align(enums::Align::Right);
            info.set_buffer(text::TextBuffer::default());

            let row_2 = Flex::default().row();
            col.set_size(&row_2, 230);

            let (crosshair_radio, explosion_input, button_group) = {
                let mut col = Flex::default_fill().column();

                let mut crosshair_radio =
                    button::RadioRoundButton::default_fill().with_label("Apply to crosshairs");
                crosshair_radio.emit(s, Message::CrosshairRadioClicked);
                crosshair_radio.toggle(true);
                col.set_size(&crosshair_radio, 20);

                let mut explosion_radio =
                    button::RadioRoundButton::default_fill().with_label("Apply to explosions");
                explosion_radio.emit(s, Message::ExplosionRadioClicked);
                col.set_size(&explosion_radio, 20);

                let explosion_input = menu::Choice::default_fill();
                col.set_size(&explosion_input, 30);

                let mut apply_btn = button::Button::default_fill().with_label("Apply");
                apply_btn.emit(s, Message::ButtonClicked(ButtonMsg::Apply));

                let mut apply_class_btn = button::Button::default_fill()
                    .with_label("...to all weapons of this class")
                    .with_align(enums::Align::Left | enums::Align::Inside);
                apply_class_btn.emit(s, Message::ButtonClicked(ButtonMsg::ToClass));
                col.set_size(&apply_class_btn, 30);

                let mut apply_slot_btn = button::Button::default_fill()
                    .with_label("...to all weapons of this slot")
                    .with_align(enums::Align::Left | enums::Align::Inside);
                apply_slot_btn.emit(s, Message::ButtonClicked(ButtonMsg::ToSlot));
                col.set_size(&apply_slot_btn, 30);

                let mut apply_all_btn = button::Button::default_fill()
                    .with_label("...to all weapons")
                    .with_align(enums::Align::Left | enums::Align::Inside);
                apply_all_btn.emit(s, Message::ButtonClicked(ButtonMsg::ToAll));
                col.set_size(&apply_all_btn, 30);

                col.end();

                let button_group = (apply_class_btn, apply_slot_btn, apply_all_btn);

                (crosshair_radio, explosion_input, button_group)
            };

            let crosshair_list = CrosshairList::new();

            row_2.end();

            col.end();
            row.end();

            (
                weapon_list,
                info,
                crosshair_radio,
                explosion_input,
                button_group,
                crosshair_list,
            )
        };

        let log = {
            let row = Flex::default().row();
            main_column.set_size(&row, 200);

            let log = Log::new();

            row.end();

            log
        };

        main_column.end();

        wind.end();
        wind.show();

        Self {
            app,

            weapon_list,
            info,
            crosshair_radio,
            explosion_input,
            button_group,
            crosshair_list,
            log,

            s,
            r,
        }
    }

    fn display_info(&mut self, weapon_file: &WeaponFile) -> Result<()> {
        let weapon = self.weapon_list.find_value(&weapon_file.name)?;

        let slot_type = match weapon.1["slot"].as_u8().unwrap() {
            1 => "Primary",
            2 => "Secondary",
            3 => "Melee",
            4 => "PDA",
            5 => "PDA",
            9 => "Other",
            _ => "",
        };

        self.info.buffer().unwrap().set_text(&format!(
            "\
Class: {}\n
Weapon Class: {}\n
Category: {}\n
Slot: {}\n
Affected Weapons:
  - {}",
            weapon.1["class"],
            weapon.0,
            weapon.1["display"],
            slot_type,
            weapon.1["all"]
                .members()
                .map(|e| e.as_str().unwrap())
                .collect::<Vec<_>>()
                .join("\n  - ")
        ));

        Ok(())
    }

    fn set_explosion_choice(&mut self, weapon_file: &WeaponFile) -> Result<()> {
        if !USES_EXPLOSION.contains(&weapon_file.name.as_str()) {
            self.explosion_input.clear();
            self.app.redraw();

            return Ok(());
        }

        if weapon_file.explosion_effect.is_none() {
            bail!("Expected an explosion type in {}", weapon_file.name);
        }

        let explosion_types = [
            ExplosionEffect::Default,
            ExplosionEffect::PyroPool,
            ExplosionEffect::MuzzleFlash,
            ExplosionEffect::SapperDestroyed,
            ExplosionEffect::ElectricShock,
        ];

        let explosion_type = weapon_file.explosion_effect.as_ref().unwrap();

        self.explosion_input.clear();

        self.explosion_input.add_choice(explosion_type.to_str());
        self.explosion_input.set_value(0);

        for e in explosion_types {
            if &e == explosion_type {
                continue;
            }

            self.explosion_input.add_choice(e.to_str());
        }

        Ok(())
    }

    fn change_crosshair(&mut self, weapon: &WeaponFile) -> Result<()> {
        let selected_crosshair = self
            .crosshair_list
            .selected()
            .ok_or_else(|| anyhow!("No crosshair selected"))?;

        let new_weapon_file = weapon.replace_crosshair(&selected_crosshair)?;

        std::fs::write(&weapon.path, new_weapon_file)?;

        self.log.log(
            LogType::Info,
            format!(
                "{}: {} -> {}",
                weapon.name,
                Path::new(&weapon.crosshair)
                    .file_stem()
                    .unwrap()
                    .to_string_lossy(),
                Path::new(&selected_crosshair.name)
                    .file_stem()
                    .unwrap()
                    .to_string_lossy(),
            ),
        );

        Ok(())
    }

    fn change_explosion(&mut self, weapon: &WeaponFile) -> Result<()> {
        let explosion = self
            .explosion_input
            .choice()
            .ok_or_else(|| anyhow!("No explosion selected"))?
            .as_str()
            .into();

        let new_weapon_file = weapon.replace_explosion(&explosion)?;

        std::fs::write(&weapon.path, new_weapon_file)?;

        self.log.log(
            LogType::Info,
            format!(
                "{}: {} -> {}",
                weapon.name,
                weapon.explosion_effect.clone().unwrap().to_str(),
                explosion.to_str()
            ),
        );

        Ok(())
    }

    fn apply_crosshairs(&mut self, weapons: Vec<(i32, WeaponFile)>) -> Result<()> {
        if weapons.is_empty() {
            bail!("No weapon selected");
        }

        for (i, weapon) in weapons {
            error_log!(self.log, self.change_crosshair(&weapon));
            error_log!(self.log, self.weapon_list.update_weapon(i, &weapon));
        }

        Ok(())
    }

    pub fn launch(&mut self) {
        std::thread::spawn({
            let mut weapon_list = self.weapon_list.clone();
            let mut crosshair_list = self.crosshair_list.clone();
            let mut log = self.log.clone();

            let s = self.s;

            move || {
                error_log!(log, weapon_list.init(&mut log, s));
                error_log!(log, crosshair_list.init(&mut log, s));
            }
        });

        while self.app.wait() {
            if let Some(msg) = self.r.recv() {
                match msg {
                    Message::WeaponListClicked => match self.weapon_list.selected() {
                        None => self.log.log(
                            LogType::Error,
                            format!(
                                "Item index `{}` doesn't have any data",
                                self.weapon_list.list.value()
                            ),
                        ),
                        Some(wf) => {
                            error_log!(self.log, self.display_info(&wf));
                            error_log!(self.log, self.set_explosion_choice(&wf));
                        }
                    },
                    Message::ButtonClicked(btn) => match btn {
                        ButtonMsg::Apply => {
                            if self.crosshair_radio.is_toggled() {
                                let all_selected = self.weapon_list.all_selected();

                                error_log!(self.log, self.apply_crosshairs(all_selected));
                            } else {
                                let weapon = match self.weapon_list.selected() {
                                    Some(w) => w,
                                    None => {
                                        self.log.log(LogType::Error, "No selected item");
                                        continue;
                                    }
                                };

                                let i = self.weapon_list.list.value();

                                if !USES_EXPLOSION.contains(&weapon.name.as_str()) {
                                    self.log.log(
                                        LogType::Error,
                                        format!("{} doesn't use explosions", weapon.name),
                                    );
                                    continue;
                                }

                                error_log!(self.log, self.change_explosion(&weapon));
                                error_log!(self.log, self.weapon_list.update_weapon(i, &weapon));
                            }
                        }
                        ButtonMsg::ToClass => {
                            let selected = match self.weapon_list.selected() {
                                Some(s) => s,
                                None => {
                                    self.log.log(LogType::Error, "No weapon selected");
                                    continue;
                                }
                            };

                            let all_class = self.weapon_list.all_class(&selected.class);

                            error_log!(self.log, self.apply_crosshairs(all_class));
                        }
                        ButtonMsg::ToSlot => {
                            let selected = match self.weapon_list.selected() {
                                Some(s) => s,
                                None => {
                                    self.log.log(LogType::Error, "No weapon selected");
                                    continue;
                                }
                            };

                            let all_slot = self.weapon_list.all_slot(selected.slot);

                            error_log!(self.log, self.apply_crosshairs(all_slot));
                        }
                        ButtonMsg::ToAll => {
                            let all_weapons = self.weapon_list.all_items();

                            error_log!(self.log, self.apply_crosshairs(all_weapons));
                        }
                    },
                    Message::CrosshairRadioClicked => {
                        self.button_group.0.activate();
                        self.button_group.1.activate();
                        self.button_group.2.activate();

                        self.weapon_list.crosshair_display(&USES_EXPLOSION);
                    }
                    Message::ExplosionRadioClicked => {
                        self.button_group.0.deactivate();
                        self.button_group.1.deactivate();
                        self.button_group.2.deactivate();

                        self.weapon_list.explosion_display(&USES_EXPLOSION);
                    }
                    Message::Redraw => self.app.redraw(),
                }
            }
        }
    }
}
