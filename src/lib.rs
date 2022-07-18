pub mod gui;

use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use gui::crosshair_list::CrosshairItem;

const ASSOCIATIONS: &str = include_str!("associations.json");

const USES_EXPLOSION: [&str; 7] = [
    "tf_weapon_rocketlauncher",
    "tf_weapon_particle_cannon",
    "tf_weapon_rocketlauncher_directhit",
    "tf_weapon_rocketlauncher_airstrike",
    "tf_weapon_grenadelauncher",
    "tf_weapon_cannon",
    "tf_weapon_pipebomblauncher",
];

#[derive(Clone, Debug, PartialEq)]
pub enum ExplosionEffect {
    Default,
    PyroPool,
    MuzzleFlash,
    SapperDestroyed,
    ElectricShock,
    Other(String),
}

impl ExplosionEffect {
    fn to_str(&self) -> &str {
        match self {
            Self::Default => "Default",
            Self::PyroPool => "Pyro Pool",
            Self::MuzzleFlash => "Muzzle Flash",
            Self::SapperDestroyed => "Sapper Destroyed",
            Self::ElectricShock => "Electric Shock",
            Self::Other(s) => s.as_str(),
        }
    }

    fn to_weapon_file_str(&self) -> &str {
        match self {
            ExplosionEffect::Default => panic!("Deal with default elsewhere"),
            ExplosionEffect::PyroPool => "eotl_pyro_pool_explosion_flash",
            ExplosionEffect::MuzzleFlash => "muzzle_minigun_starflash01",
            ExplosionEffect::SapperDestroyed => "ExplosionCore_sapperdestroyed",
            ExplosionEffect::ElectricShock => "electrocuted_red_flash",
            ExplosionEffect::Other(e) => e,
        }
    }
}

impl From<&str> for ExplosionEffect {
    fn from(s: &str) -> Self {
        match s {
            "Default" => Self::Default,
            "Pyro Pool" => Self::PyroPool,
            "Muzzle Flash" => Self::MuzzleFlash,
            "Sapper Destroyed" => Self::SapperDestroyed,
            "Electric Shock" => Self::ElectricShock,
            _ => Self::Other(s.into()),
        }
    }
}

#[derive(Clone)]
pub struct WeaponFile {
    name: String,
    path: PathBuf,
    class: String,
    slot: u8,
    crosshair: String,
    explosion_effect: Option<ExplosionEffect>,
}

impl WeaponFile {
    fn new(path: &Path, class: String, slot: u8) -> Result<Self> {
        let file_name = match path.file_name() {
            Some(f) => f.to_str().unwrap(),
            None => bail!("Invalid file name `{}`", path.display()),
        };

        if !path.exists() {
            bail!("{} doesn't exist", file_name);
        }

        let file_content =
            fs::read_to_string(path).with_context(|| format!("Failed to open {}", file_name))?;

        let mut lines = file_content.lines().map(str::trim);

        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        let mut crosshair = String::new();
        let mut explosion_effect = None;

        while let Some(line) = lines.next() {
            if USES_EXPLOSION.contains(&name.as_str()) && line.starts_with("\"ExplosionEffect\"") {
                explosion_effect = Some(
                    Self::get_value(line)
                        .with_context(|| format!("Failed to get value in {}", file_name))?,
                );
            }

            if line.starts_with("\"crosshair\"") {
                lines.next();

                for line in lines.by_ref() {
                    if line.starts_with("\"file\"") {
                        crosshair = Self::get_value(line)
                            .with_context(|| format!("Failed to get value in {}", file_name))?
                            .replace('"', "");
                        break;
                    }
                }
            }
        }

        let explosion_effect = match explosion_effect {
            Some(e) => match e.as_str() {
                "ExplosionCore_wall" => Some(ExplosionEffect::Default),
                "eotl_pyro_pool_explosion_flash" => Some(ExplosionEffect::PyroPool),
                "muzzle_minigun_starflash01" => Some(ExplosionEffect::MuzzleFlash),
                "ExplosionCore_sapperdestroyed" => Some(ExplosionEffect::SapperDestroyed),
                "electrocuted_red_flash" => Some(ExplosionEffect::ElectricShock),
                _ => Some(ExplosionEffect::Other(e)),
            },
            None => None,
        };

        Ok(Self {
            name,
            path: path.into(),
            class,
            slot,
            crosshair,
            explosion_effect,
        })
    }

    fn get_value(line: &str) -> Result<String> {
        let line = line.split_whitespace().nth(1);

        if line.is_none() {
            bail!("Failed to get value");
        }

        Ok(line.unwrap().replace('"', ""))
    }

    fn replace_value(vec: &mut Vec<String>, i: usize, value: &str) -> Result<()> {
        let v = Self::get_value(&vec[i])?;

        vec.insert(i, vec[i].replace(&v, value));
        vec.remove(i + 1);

        Ok(())
    }

    fn replace_crosshair(&self, crosshair: &CrosshairItem) -> Result<String> {
        let file_name = match self.path.file_name() {
            Some(f) => f.to_str().unwrap(),
            None => bail!("Invalid file name `{}`", self.path.display()),
        };

        if !self.path.exists() {
            bail!("{} doesn't exist", file_name);
        }

        let file_content = fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to open {}", file_name))?;

        let mut new_file = file_content
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();

        let mut lines = file_content.lines().map(str::trim).enumerate();

        while let Some((_, line)) = lines.next() {
            if line.starts_with("\"crosshair\"") {
                for (i, line) in lines.by_ref() {
                    let mut replace_value = |value| -> Result<()> {
                        Self::replace_value(&mut new_file, i, value)
                            .with_context(|| format!("Failed to replace value in {}", file_name))
                    };

                    if line.starts_with("\"file\"") {
                        replace_value(&format!(
                            "vgui/replay/thumbnails/{}",
                            crosshair.path.file_stem().unwrap().to_string_lossy()
                        ))?;
                    } else if line.starts_with("\"x\"") || line.starts_with("\"y\"") {
                        replace_value("0")?;
                    } else if line.starts_with("\"width\"") {
                        let size = if crosshair.size.0 != 0 {
                            Cow::from(crosshair.size.0.to_string())
                        } else {
                            Cow::from("64")
                        };

                        replace_value(&size)?;
                    } else if line.starts_with("\"height\"") {
                        let size = if crosshair.size.1 != 0 {
                            Cow::from(crosshair.size.1.to_string())
                        } else {
                            Cow::from("64")
                        };

                        replace_value(&size)?;
                    } else if line.contains('}') {
                        break;
                    }
                }
            }
        }

        Ok(new_file.join("\n"))
    }

    fn replace_explosion(&self, explosion: &ExplosionEffect) -> Result<String> {
        let file_name = match self.path.file_name() {
            Some(f) => f.to_str().unwrap(),
            None => bail!("Invalid file name `{}`", self.path.display()),
        };

        if !self.path.exists() {
            bail!("{} doesn't exist", file_name);
        }

        let file_content = fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to open {}", file_name))?;

        let mut new_file = file_content
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();

        let lines = file_content.lines().map(str::trim).enumerate();

        for (i, line) in lines {
            let mut replace_value = |value| -> Result<()> {
                Self::replace_value(&mut new_file, i, value)
                    .with_context(|| format!("Failed to replace value in {}", file_name))
            };

            if line.starts_with("\"ExplosionEffect\"") {
                replace_value(match explosion {
                    ExplosionEffect::Default => "ExplosionCore_wall",
                    e => e.to_weapon_file_str(),
                })?;
            } else if line.starts_with("\"ExplosionPlayerEffect\"") {
                replace_value(match explosion {
                    ExplosionEffect::Default => "ExplosionCore_MidAir",
                    e => e.to_weapon_file_str(),
                })?;
            } else if line.starts_with("\"ExplosionWaterEffect\"") {
                replace_value(match explosion {
                    ExplosionEffect::Default => "ExplosionCore_MidAir_underwater",
                    e => e.to_weapon_file_str(),
                })?;
            }
        }

        Ok(new_file.join("\n"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn associations() {
        let json = json::parse(super::ASSOCIATIONS).unwrap();

        for file in std::fs::read_dir(std::path::Path::new("resources/scripts")).unwrap() {
            let file = file.unwrap().path();

            assert!(json.has_key(file.file_stem().unwrap().to_str().unwrap()));
        }
    }

    #[test]
    fn parse() {
        let w = WeaponFile::new(
            Path::new("resources/scripts/tf_weapon_grenadelauncher.txt"),
            "Demoman".into(),
            1,
        )
        .unwrap();

        assert_eq!(w.name, "tf_weapon_grenadelauncher".to_string());
        assert_eq!(w.crosshair, "sprites/crosshairs".to_string());
        assert_eq!(w.explosion_effect, Some(ExplosionEffect::Default));

        let w_2 = WeaponFile::new(
            Path::new("resources/scripts/tf_weapon_flaregun.txt"),
            "Pyro".into(),
            2,
        )
        .unwrap();

        assert_eq!(w_2.name, "tf_weapon_flaregun".to_string());
        assert_eq!(w_2.crosshair, "sprites/crosshairs".to_string());
        assert_eq!(w_2.explosion_effect, None);
    }

    #[test]
    fn replace_crosshair() {
        let w = WeaponFile::new(
            Path::new("resources/scripts/tf_weapon_grenadelauncher.txt"),
            "Demoman".into(),
            1,
        )
        .unwrap();

        let c = CrosshairItem {
            name: "".into(),
            path: "vgui/replay/thumbnails/bigcross.vtf".into(),
            size: (64, 64),
        };

        let s = w.replace_crosshair(&c).unwrap();

        let temp_dir = tempfile::tempdir().unwrap();

        let temp_dir = temp_dir.path().join("replace_explosion.txt");

        fs::write(&temp_dir, s).unwrap();

        let w = WeaponFile::new(&temp_dir, "Demoman".into(), 1).unwrap();

        assert_eq!(w.crosshair, "vgui/replay/thumbnails/bigcross");
    }

    #[test]
    fn replace_explosion() {
        let w = WeaponFile::new(
            Path::new("resources/scripts/tf_weapon_grenadelauncher.txt"),
            "Demoman".into(),
            1,
        )
        .unwrap();

        let s = w
            .replace_explosion(&ExplosionEffect::ElectricShock)
            .unwrap();

        let temp_dir = tempfile::tempdir().unwrap();

        let temp_dir = temp_dir.path().join("tf_weapon_grenadelauncher.txt");

        fs::write(&temp_dir, s).unwrap();

        let w = WeaponFile::new(&temp_dir, "Demoman".into(), 1).unwrap();

        assert_eq!(w.explosion_effect, Some(ExplosionEffect::ElectricShock));
    }
}
