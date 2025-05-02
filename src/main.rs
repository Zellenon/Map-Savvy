#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::MyApp;
use eframe::NativeOptions;

mod app;
mod map_generator;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Map Savvy",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
