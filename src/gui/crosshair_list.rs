use crate::gui::{Log, LogType, Message};

use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use fltk::{app::Sender, browser, group, prelude::*};

#[derive(Clone)]
pub struct CrosshairItem {
    pub name: String,
    pub path: PathBuf,
    pub size: (i32, i32),
}

#[derive(Clone)]
pub(crate) struct CrosshairList {
    list: browser::SelectBrowser,
}

impl CrosshairList {
    pub fn new() -> Self {
        let row = group::Flex::default_fill().row();

        let list = browser::SelectBrowser::default_fill();

        row.end();

        Self { list }
    }

    pub fn init(&mut self, log: &mut Log, s: Sender<Message>) -> Result<()> {
        let mut crosshair_dir =
            std::env::current_exe().with_context(|| "Failed to get current executable path")?;
        crosshair_dir.pop();
        crosshair_dir.push("materials/vgui/replay/thumbnails");

        if !crosshair_dir.exists() {
            bail!("Failed to find `materials/vgui/replay/thumbnails` folder");
        }

        for file in crosshair_dir.read_dir().with_context(|| {
            format!(
                "Failed to read folder `{}`",
                crosshair_dir.file_name().unwrap().to_string_lossy()
            )
        })? {
            match file {
                Ok(crosshair) => {
                    if crosshair.path().extension() == Some(std::ffi::OsStr::new("vtf")) {
                        let crosshair_name = crosshair.file_name().to_string_lossy().into_owned();

                        self.list.add_with_data(
                            &crosshair_name,
                            CrosshairItem {
                                name: crosshair_name.clone(),
                                path: crosshair.path(),
                                size: (0, 0),
                            },
                        );
                    }
                }
                Err(e) => log.log(LogType::Error, e),
            }
        }

        for i in 1..=self.list.size() {
            let crosshair = self.get_item(i).unwrap();

            match self.vtf_to_image(&crosshair.path) {
                Ok(mut image) => {
                    self.list.set_data(
                        i,
                        CrosshairItem {
                            size: (image.width(), image.height()),
                            ..crosshair
                        },
                    );

                    image.scale(32, 32, true, true);
                    self.list.set_icon(i, Some(image));
                }
                Err(e) => {
                    log.log(
                        LogType::Error,
                        format!("Skipping {}; {}", crosshair.name, e),
                    );
                }
            }
        }

        s.send(Message::Redraw);

        Ok(())
    }

    fn vtf_to_image(&mut self, vtf_path: &Path) -> Result<fltk::image::PngImage> {
        let mut crosshair_file = std::fs::File::open(&vtf_path)?;

        let mut buf = Vec::new();

        crosshair_file.read_to_end(&mut buf)?;

        let vtf = vtf::from_bytes(&mut buf)?.highres_image.decode(0)?;

        buf.clear();

        vtf.write_to(&mut buf, image::ImageFormat::PNG)?;

        Ok(fltk::image::PngImage::from_data(&buf).unwrap())
    }

    fn get_item(&self, i: i32) -> Option<CrosshairItem> {
        unsafe { self.list.data(i) }
    }

    pub fn selected(&self) -> Option<CrosshairItem> {
        self.get_item(self.list.value())
    }
}
