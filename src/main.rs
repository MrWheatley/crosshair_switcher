#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    let mut app = crosshair_switcher::gui::App::new("crosshair-switcher");
    app.launch();
}
