#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod gui;

fn main() {
    let mut app = gui::App::new();
    app.launch();
}
