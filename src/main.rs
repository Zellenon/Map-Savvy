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

fn main2() {
    let imgx = 1000;
    let imgy = 1000;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
