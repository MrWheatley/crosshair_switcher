use crate::gui::{Log, LogType, Message};
use crate::{WeaponFile, ASSOCIATIONS};

use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use fltk::app::Sender;
use fltk::{app, browser, group, prelude::*};

#[derive(Clone)]
pub struct WeaponList {
    pub list: browser::MultiBrowser,
    json_data: json::JsonValue,
}

impl WeaponList {
    pub fn new(s: app::Sender<Message>) -> Self {
        let row = group::Flex::default_fill().row();

        let mut list = browser::MultiBrowser::default_fill();
        list.emit(s, Message::WeaponListClicked);
        list.set_column_widths(&[85, 190]);

        row.end();

        Self {
            list,
            json_data: json::parse(ASSOCIATIONS).expect("Failed to parse json"),
        }
    }

    pub fn init(&mut self, log: &mut Log, s: Sender<Message>) -> Result<()> {
        let mut scripts_dir =
            std::env::current_exe().with_context(|| "Failed to get current executable path")?;
        scripts_dir.pop();
        scripts_dir.push("scripts");

        if !scripts_dir.exists() {
            bail!("Failed to find `scripts` folder");
        }

        for weapon in self.json_data.entries() {
            let weapon_file = match WeaponFile::new(
                &scripts_dir.join(weapon.0).as_path().with_extension("txt"),
                weapon.1["class"].to_string(),
                weapon.1["slot"].as_u8().unwrap(),
            ) {
                Ok(wf) => wf,
                Err(e) => {
                    log.log(LogType::Error, format!("Skipping {}; {}", weapon.0, e));
                    continue;
                }
            };

            self.list.add_with_data(
                &format!(
                    "@f{}\t@f{}\t@f{}",
                    weapon.1["class"],
                    weapon.1["display"],
                    &Path::new(&weapon_file.crosshair)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                ),
                weapon_file,
            );
        }

        s.send(Message::Redraw);

        Ok(())
    }

    pub fn update_weapon(&mut self, i: i32, weapon: &WeaponFile) -> Result<()> {
        let old = self.get_item(i).unwrap();
        let new_weapon_file = WeaponFile::new(&weapon.path, old.class, old.slot)?;

        let text = self.list.text(i).unwrap();

        let mut text = text.split("@f");
        text.next();

        self.list.set_text(
            i,
            &format!(
                "@f{}@f{}@f{}",
                text.next().unwrap(),
                text.next().unwrap(),
                &Path::new(&new_weapon_file.crosshair)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
            ),
        );

        self.list.set_data(i, new_weapon_file);

        Ok(())
    }

    pub fn crosshair_display(&mut self, weapon_list: &[&str]) {
        for (i, weapon) in self
            .all_items()
            .into_iter()
            .filter(|w| weapon_list.contains(&w.1.name.as_str()))
        {
            let text = self.list.text(i).unwrap();

            if !text.starts_with("@f>") {
                return;
            }

            let text = text.replace("> ", "");
            let mut text = text.split("@f");
            text.next();

            self.list.set_text(
                i,
                &format!(
                    "@f{}@f{}@f{}",
                    text.next().unwrap(),
                    text.next().unwrap(),
                    &Path::new(&weapon.crosshair)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                ),
            );
        }
    }

    pub fn explosion_display(&mut self, weapon_list: &[&str]) {
        for (i, weapon) in self
            .all_items()
            .into_iter()
            .filter(|w| weapon_list.contains(&w.1.name.as_str()))
        {
            let text = self.list.text(i).unwrap();

            if text.starts_with("@f>") {
                return;
            }

            let mut text = text.split("@f");
            text.next();

            self.list.set_text(
                i,
                &format!(
                    "@f> {}@f{}@f{}",
                    text.next().unwrap(),
                    text.next().unwrap(),
                    &Path::new(&weapon.crosshair)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                ),
            );
        }
    }
    pub fn find_value(&self, key: &str) -> Result<(&str, &json::JsonValue)> {
        self.json_data
            .entries()
            .find(|e| e.0 == key)
            .ok_or_else(|| anyhow!("Key `{}` not in JSON data", key))
    }

    pub fn get_item(&self, i: i32) -> Option<WeaponFile> {
        unsafe { self.list.data(i) }
    }

    pub fn selected(&self) -> Option<WeaponFile> {
        self.get_item(self.list.value())
    }

    pub fn all_selected(&self) -> Vec<(i32, WeaponFile)> {
        (1..=self.list.size())
            .filter(|i| self.list.selected(*i))
            .map(|i| (i, self.get_item(i)))
            .filter(|(_, w)| w.is_some())
            .map(|(i, w)| (i, w.unwrap()))
            .collect()
    }

    pub fn all_class(&self, class: &str) -> Vec<(i32, WeaponFile)> {
        self.all_items()
            .into_iter()
            .filter(|w| w.1.class == class)
            .collect()
    }

    pub fn all_slot(&self, slot: u8) -> Vec<(i32, WeaponFile)> {
        self.all_items()
            .into_iter()
            .filter(|w| w.1.slot == slot)
            .collect()
    }

    pub fn all_items(&self) -> Vec<(i32, WeaponFile)> {
        (1..=self.list.size())
            .into_iter()
            .map(|i| (i, self.get_item(i)))
            .filter(|(_, w)| w.is_some())
            .map(|(i, w)| (i, w.unwrap()))
            .collect()
    }
}
