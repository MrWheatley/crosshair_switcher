use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use zip_extensions::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    run()?;

    Ok(())
}

fn run() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => dist()?,
        _ => print_help(),
    }

    Ok(())
}

fn print_help() {
    eprintln!(
        "\
TASKS:
    dist            Builds the binary and zips them with the files in `resources`
"
    )
}

fn dist() -> Result<()> {
    if dist_dir().exists() {
        fs::remove_dir_all(dist_dir())?;
    }

    fs::create_dir_all(dist_dir())?;

    dist_binary()?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn dist_binary() -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    let dst = project_root().join("target/release/crosshair-switcher.exe");

    fs::copy(&dst, dist_dir().join("crosshair-switcher.exe"))?;

    let clean_path = |path: &Path| {
        let p = path.to_str().unwrap();

        if p.starts_with("\\\\?\\") {
            return p.strip_prefix("\\\\?\\").unwrap().to_string();
        }
        p.to_string()
    };

    let from_scripts_dir = clean_path(&project_root().join("resources/scripts").canonicalize()?);
    let from_materials_dir =
        clean_path(&project_root().join("resources/materials").canonicalize()?);

    let to_scripts_dir = dist_dir().join("scripts");
    let to_materials_dir = dist_dir().join("materials");

    fs::create_dir_all(&to_scripts_dir)?;
    fs::create_dir_all(&to_materials_dir)?;

    let to_scripts_dir = clean_path(&to_scripts_dir.canonicalize()?);
    let to_materials_dir = clean_path(&to_materials_dir.canonicalize()?);

    let status = Command::new("xcopy.exe")
        .current_dir(project_root())
        .args(&[from_scripts_dir, to_scripts_dir])
        .args(&["/E", "/H", "/I"])
        .status()?;

    if !status.success() {
        Err("copying scripts dir failed")?;
    }

    let status = Command::new("xcopy.exe")
        .current_dir(project_root())
        .args(&[from_materials_dir, to_materials_dir])
        .args(&["/E", "/H", "/I"])
        .status()?;

    if !status.success() {
        Err("copying materials dir failed")?;
    }

    let archive_file = dist_dir().parent().unwrap().join("crosshair-switcher.zip");
    let source_dir = dist_dir();

    zip_create_from_directory(&archive_file, &source_dir)?;

    fs::copy(&archive_file, dist_dir().join("crosshair-switcher.zip"))?;
    fs::remove_file(&archive_file)?;

    Ok(())
}

#[cfg(target_os = "linus")]
fn dist_binary() -> Result<()> {
    todo!()
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}
