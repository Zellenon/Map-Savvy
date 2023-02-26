use image::ImageError;
use rand::prelude::*;
use rayon::prelude::*;
use std::f64::consts::PI;

const RED: [u8; 49] = [
    0, 0, 0, 0, 0, 0, 0, 0, 34, 68, 102, 119, 136, 153, 170, 187, 0, 34, 34, 119, 187, 255, 238,
    221, 204, 187, 170, 153, 136, 119, 85, 68, 255, 250, 245, 240, 235, 230, 225, 220, 215, 210,
    205, 200, 195, 190, 185, 180, 175,
];
const GREEN: [u8; 49] = [
    0, 0, 17, 51, 85, 119, 153, 204, 221, 238, 255, 255, 255, 255, 255, 255, 68, 102, 136, 170,
    221, 187, 170, 136, 136, 102, 85, 85, 68, 51, 51, 34, 255, 250, 245, 240, 235, 230, 225, 220,
    215, 210, 205, 200, 195, 190, 185, 180, 175,
];
const BLUE: [u8; 49] = [
    0, 68, 102, 136, 170, 187, 221, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 34,
    34, 34, 34, 34, 34, 34, 34, 34, 17, 0, 255, 250, 245, 240, 235, 230, 225, 220, 215, 210, 205,
    200, 195, 190, 185, 180, 175,
];

#[derive(Clone)]
pub struct MapData {
    pub faults: Vec<Fault>,
    pub seed: u32,
    pub seed_name: String,
    pub percent_water: f64,
    pub size: (u32, u32),
}

impl Default for MapData {
    fn default() -> Self {
        MapData {
            faults: Vec::default(),
            seed: 0,
            seed_name: String::new(),
            percent_water: 0.6,
            size: (3000, 2000),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Fault {
    flag: bool,
    alpha: f64,
    beta: f64,
    shift: f64,
    tan_b: f64,
    xsi: f64,
}

impl Fault {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let alpha = (rng.gen::<f64>() - 0.5) * PI;
        let beta = (rng.gen::<f64>() - 0.5) * PI;
        Self {
            flag: rng.gen::<bool>(),
            alpha,
            beta,
            shift: (rng.gen::<f64>() - 0.5),
            tan_b: (alpha.cos() * beta.cos()).acos().tan(),
            xsi: (0.5 - beta / PI),
        }
    }
}

fn map_color(i: usize) -> image::Rgb<u8> {
    image::Rgb([RED[i], GREEN[i], BLUE[i]])
}

pub fn map_image(data: &MapData) -> Result<egui::ColorImage, ImageError> {
    let image_width = data.size.0 as f64;
    let image_height = data.size.1 as f64;

    let y_range_div_2 = (image_height as f64) / 2.;
    let y_range_div_pi = (image_height as f64) / PI;
    let sin_iter_phi = |x: f64| (x * 2. * PI / image_width).sin();

    let world_shape: Vec<Vec<(u32, u32)>> = (0..data.size.0)
        .map(|w| (0..data.size.1).map(|w2| (w, w2)).collect())
        .collect();

    let world_heights: Vec<Vec<f64>> = world_shape
        .par_iter()
        .enumerate()
        .map(|(x, column)| {
            let flag_theta: Vec<(bool, f64)> = data
                .faults
                .par_iter()
                .map(|fault_i| {
                    let sin_iter_index = image_width * (fault_i.xsi + fault_i.shift) - x as f64;
                    let atan_args = sin_iter_phi(sin_iter_index) * fault_i.tan_b;
                    (
                        fault_i.flag,
                        y_range_div_pi * atan_args.atan() + y_range_div_2, // Theta
                    )
                })
                .collect();
            column
                .par_iter()
                .enumerate()
                .map(|(y, _)| {
                    flag_theta
                        .par_iter()
                        .map(|(a, theta)| {
                            let b = y as f64 <= *theta;
                            // This should be XOR, but this is the same and faster
                            if *a == b {
                                1.
                            } else {
                                -1.
                            }
                        })
                        .sum()
                })
                .collect()
        })
        .collect();
    println!("Finished calculating world heights");

    let world_max = world_heights
        .iter()
        .map(|w| w.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
        .fold(f64::NEG_INFINITY, f64::max);
    let world_min = world_heights
        .iter()
        .map(|w| w.iter().cloned().fold(f64::INFINITY, f64::min))
        .fold(f64::INFINITY, f64::min);
    let world_range = world_max - world_min;

    let mut imgbuf = image::ImageBuffer::new(data.size.0, data.size.1);
    imgbuf.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let i = (world_heights[x as usize][y as usize] as f64 - world_min) / world_range * 30.;
        *pixel = map_color(i as usize);
    });
    println!("Colors set");

    imgbuf.save("map.png").unwrap();
    // let size = [imgbuf.width() as _, imgbuf.height() as _];
    // let pixels = imgbuf.as_flat_samples();
    // ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
    let image = image::io::Reader::open("fractal.png")?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
