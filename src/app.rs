use eframe::egui;
use egui::{ColorImage, Slider, TextEdit, TextureHandle};
use poll_promise::Promise;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::sync::Arc;

use crate::map_generator::{map_image, Fault, MapData};

enum AppState {
    Startup,
    Generating(Promise<(MapData, ColorImage)>),
    ImageGenerated((Arc<ColorImage>, TextureHandle)),
}

pub struct MyApp {
    generation_state: AppState,
    generator_options: GeneratorOptions,
    map_data: MapData,
}

#[derive(Clone, Debug)]
pub struct GeneratorOptions {
    n_faults: usize,
    seed: String,
    percent_water: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            generation_state: AppState::Startup,
            generator_options: GeneratorOptions {
                n_faults: 20,
                seed: "seed".to_owned(),
                percent_water: 55,
            },
            map_data: MapData::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &self.generation_state {
            AppState::Startup => {}
            AppState::Generating(promise) => {
                if let Some((_data, img)) = promise.ready() {
                    let image = Arc::new(img.clone());
                    let handle =
                        ctx.load_texture("map", image.clone(), egui::TextureOptions::default());
                    self.generation_state = AppState::ImageGenerated((image, handle))
                }
            }
            AppState::ImageGenerated(_) => {}
        };
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    if ui.button("Generate").clicked() {
                        self.generate_button()
                    }
                    ui.horizontal(|ui| {
                        ui.add(Slider::new(&mut self.generator_options.n_faults, 1..=50))
                    });
                    ui.add(TextEdit::singleline(&mut self.generator_options.seed));
                    ui.horizontal(|ui| {
                        ui.add(Slider::new(
                            &mut self.generator_options.percent_water,
                            1..=99,
                        ))
                    });
                    match &self.generation_state {
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
        let n_faults = self.generator_options.n_faults;
        let mut data = MapData {
            seed_name: self.generator_options.seed.clone(),
            percent_water: (self.generator_options.percent_water as f64) / 100.,
            ..Default::default()
        };

        self.generation_state =
            AppState::Generating(Promise::spawn_thread("bg_thread", move || {
                data.faults = (0..(n_faults * 100))
                    .par_bridge()
                    .map(|_| Fault::new())
                    .collect();
                println!("Faults Complete");
                let img = map_image(&data).unwrap();
                (data, img)
            }));
    }
}
