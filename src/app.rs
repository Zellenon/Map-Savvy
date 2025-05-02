use eframe::egui;
use egui::{ColorImage, TextureHandle};
use poll_promise::Promise;
use std::sync::Arc;

use crate::map_generator::{map_image, Fault, MapData};

enum AppState {
    Startup,
    Generating(Promise<(MapData, ColorImage)>),
    ImageGenerated((Arc<ColorImage>, TextureHandle)),
}

pub struct MyApp {
    state: AppState,
    map_data: MapData,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: AppState::Startup,
            map_data: MapData::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &self.state {
            AppState::Startup => {}
            AppState::Generating(promise) => {
                if let Some((_data, img)) = promise.ready() {
                    let image = Arc::new(img.clone());
                    let handle =
                        ctx.load_texture("map", image.clone(), egui::TextureOptions::default());
                    self.state = AppState::ImageGenerated((image, handle))
                }
            }
            AppState::ImageGenerated(_) => todo!(),
        };
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    if ui.button("Generate").clicked() {
                        self.generate_button()
                    }
                    match &self.state {
                        AppState::Startup => ui.label("No Image"),
                        AppState::Generating(_) => ui.spinner(),
                        AppState::ImageGenerated((_, handle)) => ui.image(handle),
                    };
                });
            });
        });
    }
}

impl MyApp {
    fn generate_button(&mut self) {
        let mut data = self.map_data.clone();
        self.state = AppState::Generating(Promise::spawn_thread("bg_thread", move || {
            for _ in 0..2000 {
                data.faults.push(Fault::new());
            }
            println!("Faults Complete");
            let img = map_image(&data).unwrap();
            (data, img)
        }));
    }
}
