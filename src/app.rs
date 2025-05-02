use std::sync::Arc;

use eframe::egui;
use egui::{ColorImage, TextureHandle};
use poll_promise::Promise;

use crate::map_generator::{map_image, Fault, MapData};

pub struct MyApp {
    returned_map_image: Option<Promise<(MapData, ColorImage)>>,
    held_map_image: Option<Arc<ColorImage>>,
    handle: Option<TextureHandle>,
    map_data: MapData,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            returned_map_image: None,
            held_map_image: None,
            handle: None,
            map_data: MapData::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(promise) = &self.returned_map_image {
            if let Some((_data, img)) = promise.ready() {
                self.held_map_image = Some(Arc::new(img.clone()));
                self.handle = Some(ctx.load_texture(
                    "map",
                    self.held_map_image.clone().unwrap(),
                    egui::TextureOptions::default(),
                ));
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    if ui.button("Generate").clicked() {
                        self.generate_button()
                    }

                    if self.returned_map_image.is_some() {
                        match &self.handle {
                            Some(handle) => ui.image(handle),
                            None => ui.spinner(),
                        };
                    } else {
                        ui.label("No Image");
                    }
                });
            });
        });
    }
}

impl MyApp {
    fn generate_button(&mut self) {
        let mut data = self.map_data.clone();
        self.held_map_image = None;
        self.returned_map_image = Some(Promise::spawn_thread("bg_thread", move || {
            for _ in 0..2000 {
                data.faults.push(Fault::new());
            }
            println!("Faults Complete");
            let img = map_image(&data).unwrap();
            (data, img)
        }));
    }
}
