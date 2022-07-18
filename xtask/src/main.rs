use std::{env, fs, process::Command};

use normpath::{BasePathBuf, PathExt};
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

    let from_scripts_dir = project_root().join("resources/scripts");
    let from_materials_dir = project_root().join("resources/materials");

    let to_scripts_dir = dist_dir().join("scripts");
    let to_materials_dir = dist_dir().join("materials");

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

    let archive_file = dist_dir().parent()?.unwrap().join("crosshair-switcher.zip");
    let source_dir = dist_dir();

    zip_create_from_directory(
        &archive_file.as_path().to_owned(),
        &source_dir.as_path().to_owned(),
    )?;

    fs::copy(&archive_file, dist_dir().join("crosshair-switcher.zip"))?;
    fs::remove_file(&archive_file)?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn dist_binary() -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    let dst = project_root().join("target/release/crosshair-switcher");

    fs::copy(&dst, dist_dir().join("crosshair-switcher"))?;

    let from_scripts_dir = project_root().join("resources/scripts");
    let from_materials_dir = project_root().join("resources/materials");

    let to_scripts_dir = dist_dir().join("scripts");
    let to_materials_dir = dist_dir().join("materials");

    let status = Command::new("cp")
        .current_dir(project_root())
        .arg("-R")
        .args(&[from_scripts_dir, to_scripts_dir])
        .status()?;

    if !status.success() {
        Err("copying scripts dir failed")?;
    }

    let status = Command::new("cp")
        .current_dir(project_root())
        .arg("-R")
        .args(&[from_materials_dir, to_materials_dir])
        .status()?;

    if !status.success() {
        Err("copying materials dir failed")?;
    }

    let archive_file = dist_dir().parent()?.unwrap().join("crosshair-switcher.zip");
    let source_dir = dist_dir();

    zip_create_from_directory(
        &archive_file.as_path().to_owned(),
        &source_dir.as_path().to_owned(),
    )?;

    fs::copy(&archive_file, dist_dir().join("crosshair-switcher.zip"))?;
    fs::remove_file(&archive_file)?;

    Ok(())
}

fn project_root() -> BasePathBuf {
    std::path::Path::new(&env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
        .normalize()
        .unwrap()
}

fn dist_dir() -> BasePathBuf {
    project_root().join("target/dist")
}
