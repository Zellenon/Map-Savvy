#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::MyApp;
use eframe::egui;

mod app;
mod map_generator;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        decorated: true,
        // To have rounded corners we need transparency:
        transparent: true,
        min_window_size: Some(egui::vec2(400.0, 100.0)),
        initial_window_size: Some(egui::vec2(1500.0, 900.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Custom window frame", // unused title
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
