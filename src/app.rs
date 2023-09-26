use eframe::egui;
use egui_extras::RetainedImage;
use poll_promise::Promise;

use crate::map_generator::{map_image, Fault, MapData};

pub struct MyApp {
    image: RetainedImage,
    map_image: Option<Promise<(MapData, RetainedImage)>>,
    map_data: MapData,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image: RetainedImage::from_image_bytes(
                "rust-logo-256x256.png",
                include_bytes!("rust-logo-256x256.png"),
            )
            .unwrap(),
            map_image: None,
            map_data: MapData::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("This is an image:");
                    self.image.show(ui);
                    if ui.button("Generate").clicked() {
                        let mut data = self.map_data.clone();
                        self.map_image = Some(poll_promise::Promise::spawn_thread(
                            "bg_thread",
                            move || {
                                for _ in 0..2000 {
                                    data.faults.push(Fault::new());
                                }
                                println!("Faults Complete");
                                let img = map_image(&data).unwrap();
                                (data, RetainedImage::from_color_image("test", img))
                            },
                        ));
                    }
                });
                if let Some(promise) = &self.map_image {
                    if let Some((_data, img)) = promise.ready() {
                        img.show(ui);
                    } else {
                        ui.spinner();
                    }
                } else {
                    ui.label("No Image");
                }
            });
        });
    }
}
