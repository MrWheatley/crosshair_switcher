#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let mut app = crosshair_switcher::gui::App::new("crosshair-switcher");
    app.launch();
}
