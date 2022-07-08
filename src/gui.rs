use std::fmt::format;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use fltk::{app, browser, button, enums, frame, group, misc, prelude::*, text, window::Window};
use image;

const ASSOCIATIONS: &str = include_str!("associations.json");

enum LogType {
    Info,
    Error,
}

#[derive(Clone)]
struct WeaponItem {
    file: String,
    crosshair: String,
}

#[derive(Clone)]
struct CrosshairItem {
    name: String,
    path: PathBuf,
}
#[derive(Clone, Copy)]
enum Message {
    ApplyBtnClicked,
    ListClicked,
}

pub struct App {
    app: app::App,
    weapon_list: browser::MultiBrowser,
    info: text::TextDisplay,
    log: text::TextDisplay,
    crosshair_input: browser::SelectBrowser,
    explosion_input: misc::InputChoice,
    r: app::Receiver<Message>,

    json_data: json::JsonValue,
}

impl App {
    pub fn new() -> Self {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
        let theme = fltk_theme::ColorTheme::new(fltk_theme::color_themes::BLACK_THEME);
        theme.apply();

        let mut wind = Window::default()
            .with_size(900, 700)
            .with_label("crosshair-switcher")
            .center_screen();
        wind.size_range(900, 600, 0, 0);
        wind.make_resizable(true);

        let (s, r) = app::channel::<Message>();

        let mut main_colum = group::Flex::default_fill().column();
        main_colum.set_margin(5);

        let main_row = group::Flex::default_fill().row();

        let weapon_list = {
            let mut list_row = group::Flex::default_fill().row();
            list_row.set_margin(5);

            let mut list = browser::MultiBrowser::default_fill();
            list.emit(s, Message::ListClicked);
            list.set_column_widths(&[85, 190]);
            list_row.end();

            list
        };

        let (crosshair_input, explosion_input, info) = {
            let mut right_colum = group::Flex::default_fill().column();
            right_colum.set_margin(5);

            let mut info = text::TextDisplay::default_fill();
            info.set_text_font(enums::Font::Courier);
            info.set_scrollbar_align(enums::Align::Right);
            info.set_buffer(text::TextBuffer::default());

            let (crosshair_input, explosion_input) = {
                let row = group::Flex::default().row();
                right_colum.set_size(&row, 190);

                let mut column = group::Flex::default_fill().column();

                let explosion_input = misc::InputChoice::default_fill();

                {
                    frame::Frame::default().with_align(enums::Align::Bottom);

                    let mut apply_btn = button::Button::default_fill().with_label("Apply");
                    apply_btn.emit(s, Message::ApplyBtnClicked);

                    let apply_class_btn = button::Button::default_fill()
                        .with_label("...to all weapons of this class")
                        .with_align(enums::Align::Left | enums::Align::Inside);
                    let apply_slot_btn = button::Button::default_fill()
                        .with_label("...to all weapons of this slot")
                        .with_align(enums::Align::Left | enums::Align::Inside);
                    let apply_all_btn = button::Button::default_fill()
                        .with_label("...to all weapons")
                        .with_align(enums::Align::Left | enums::Align::Inside);

                    column.set_size(&explosion_input, 30);
                    column.set_size(&apply_btn, 50);
                    column.set_size(&apply_class_btn, 30);
                    column.set_size(&apply_slot_btn, 30);
                    column.set_size(&apply_all_btn, 30);
                }
                column.end();

                let crosshair_input = browser::SelectBrowser::default_fill();

                row.end();

                (crosshair_input, explosion_input)
            };

            right_colum.end();

            (crosshair_input, explosion_input, info)
        };

        main_row.end();

        let log = {
            let mut log_row = group::Flex::default().row();
            log_row.set_margin(5);
            main_colum.set_size(&log_row, 200);

            let mut log = text::TextDisplay::default_fill();
            log.set_text_font(enums::Font::Courier);
            log.set_buffer(text::TextBuffer::default());
            log.set_scrollbar_align(enums::Align::Right);

            log_row.end();

            log
        };

        main_colum.end();

        wind.end();
        wind.show();

        Self {
            app,
            weapon_list,
            info,
            log,
            crosshair_input,
            explosion_input,
            r,

            json_data: json::parse(ASSOCIATIONS).expect("Failed to parse json"),
        }
    }

    pub fn launch(&mut self) {
        if let Err(e) = self.init_list() {
            self.log(LogType::Error, &e.to_string());
        }

        if let Err(e) = self.init_crosshairs() {
            self.log(LogType::Error, &e.to_string());
        }

        while self.app.wait() {
            if let Some(msg) = self.r.recv() {
                match msg {
                    Message::ApplyBtnClicked => {
                        let selected_weapons = self.get_selected_weapons();

                        if selected_weapons.is_empty() {
                            self.log(LogType::Info, "No weapon selected");
                            continue;
                        }

                        for item in selected_weapons {
                            // TODO: apply change to weapon files
                            self.log(LogType::Info, &item.file);
                        }
                    }
                    Message::ListClicked => {
                        if let Err(e) = self.display_info() {
                            self.log(LogType::Error, &e.to_string());
                        }
                    }
                }
            }
        }
    }

    fn display_info(&mut self) -> Result<()> {
        let mut buffer = self.info.buffer().unwrap();
        let selected = match self.get_list_item(self.weapon_list.value()) {
            None => bail!(
                "Item index `{}` doesn't have data",
                self.weapon_list.value()
            ),
            Some(item) => item,
        };

        if !self.json_data.has_key(&selected.file) {
            bail!("Not in lookup: {}", selected.file);
        }

        let weapon = self
            .json_data
            .entries()
            .find(|e| e.0 == selected.file)
            .unwrap();

        let slot_type = match weapon.1["slot"].as_u8().unwrap() {
            1 => "Primary",
            2 => "Secondary",
            3 => "Melee",
            4 => "PDA",
            5 => "PDA",
            9 => "Other",
            _ => "",
        };

        buffer.set_text(&format!(
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
                .collect::<Vec<&str>>()
                .join("\n  - ")
        ));

        Ok(())
    }

    fn log(&mut self, log_type: LogType, msg: &str) {
        self.log.buffer().unwrap().append(&format!(
            "[{}] {}\n",
            match log_type {
                LogType::Info => "Info",
                LogType::Error => "Error",
            },
            msg
        ));

        self.log.scroll(
            self.log
                .count_lines(0, self.log.buffer().unwrap().length(), true),
            0,
        );
    }

    fn get_list_item(&self, i: i32) -> Option<WeaponItem> {
        unsafe { self.weapon_list.data::<WeaponItem>(i) }
    }

    fn get_crosshair_item(&self, i: i32) -> Option<CrosshairItem> {
        unsafe { self.crosshair_input.data::<CrosshairItem>(i) }
    }

    fn get_selected_weapons(&self) -> Vec<WeaponItem> {
        (1..=self.weapon_list.size())
            .filter(|&i| self.weapon_list.selected(i))
            .filter_map(|i| self.get_list_item(i))
            .collect()
    }

    fn init_list(&mut self) -> Result<()> {
        let mut scripts_dir =
            std::env::current_exe().with_context(|| "Failed to get current executable path")?;
        scripts_dir.pop();
        scripts_dir.push("scripts");

        if !scripts_dir.exists() {
            bail!("Failed to find `scripts` folder");
        }

        let mut errors = Vec::new();

        for weapon in self.json_data.entries() {
            let crosshair = match Self::parse_crosshair(
                &scripts_dir.join(weapon.0).as_path().with_extension("txt"),
            ) {
                Ok(crosshair) => crosshair,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            self.weapon_list.add_with_data(
                &format!(
                    "@f{}\t@f{}\t@f{}",
                    weapon.1["class"],
                    weapon.1["display"],
                    &Path::new(&crosshair).file_name().unwrap().to_str().unwrap()
                ),
                WeaponItem {
                    file: weapon.0.to_string(),
                    crosshair,
                },
            );
        }

        errors
            .iter()
            .for_each(|e| self.log(LogType::Error, &e.to_string()));

        Ok(())
    }

    fn init_crosshairs(&mut self) -> Result<()> {
        let mut crosshair_dir =
            std::env::current_exe().with_context(|| "Failed to get current executable path")?;
        crosshair_dir.pop();
        crosshair_dir.push("materials/vgui/replay/thumbnails");

        if !crosshair_dir.exists() {
            bail!("Failed to find `{}` folder", crosshair_dir.display());
        }

        for file in crosshair_dir
            .read_dir()
            .with_context(|| format!("Failed to read folder `{}`", crosshair_dir.display()))?
        {
            match file {
                Ok(crosshair) => {
                    if crosshair.path().extension() == Some(std::ffi::OsStr::new("vtf")) {
                        let crosshair_name = crosshair.file_name().to_str().unwrap().to_string();
                        self.crosshair_input.add_with_data(
                            &crosshair_name,
                            CrosshairItem {
                                name: crosshair_name.clone(),
                                path: crosshair.path(),
                            },
                        );
                    }
                }
                Err(e) => self.log(LogType::Error, &e.to_string()),
            }
        }

        for i in 1..=self.crosshair_input.size() {
            let crosshair = self.get_crosshair_item(i).unwrap();

            let mut crosshair_file = match std::fs::File::open(&crosshair.path) {
                Ok(file) => file,
                Err(e) => {
                    self.log(LogType::Error, &e.to_string());
                    continue;
                }
            };

            let mut buf = Vec::new();

            if let Err(e) = crosshair_file.read_to_end(&mut buf) {
                self.log(LogType::Error, &e.to_string());
            }

            let vtf = match vtf::from_bytes(&mut buf) {
                Ok(vtf) => vtf,
                Err(e) => {
                    self.log(LogType::Error, &e.to_string());
                    continue;
                }
            };

            let vtf = match vtf.highres_image.decode(0) {
                Ok(vtf) => vtf,
                Err(e) => {
                    self.log(
                        LogType::Error,
                        &format!(
                            "{}, skipping `{}`",
                            e,
                            crosshair.path.file_name().unwrap().to_str().unwrap()
                        ),
                    );
                    continue;
                }
            };

            let mut image = Vec::new();
            if let Err(e) = vtf.write_to(&mut image, image::ImageFormat::PNG) {
                self.log(LogType::Error, &e.to_string());
                continue;
            }

            let mut image = fltk::image::PngImage::from_data(&image).unwrap();
            image.scale(32, 32, true, true);

            self.crosshair_input.set_icon(i, Some(image));
        }

        Ok(())
    }

    fn parse_crosshair(file: &Path) -> Result<String> {
        if !file.exists() {
            bail!("{:?} doesn't exist", file.file_name().unwrap());
        }

        let file_content = std::fs::read_to_string(file)
            .with_context(|| format!("Failed to open {}", file.display()))?;

        let mut lines = file_content.lines().map(str::trim);

        let mut crosshair_file = String::new();

        'outer: while let Some(line) = lines.next() {
            if line.starts_with("\"crosshair\"") {
                lines.next();

                for line in lines.by_ref() {
                    if line.starts_with("\"file\"") {
                        crosshair_file = line
                            .split_whitespace()
                            .nth(1)
                            .unwrap()
                            .chars()
                            .filter(|c| c != &'"')
                            .collect();

                        break 'outer;
                    }
                }
            }
        }

        Ok(crosshair_file)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn associations() {
        let json = json::parse(super::ASSOCIATIONS).unwrap();

        for file in std::fs::read_dir(std::path::Path::new("scripts")).unwrap() {
            let file = file.unwrap().path();

            assert!(json.has_key(file.file_stem().unwrap().to_str().unwrap()));
        }
    }
}
