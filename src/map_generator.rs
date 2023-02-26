use cached::proc_macro::cached;
use image::ImageError;
use ndarray::prelude::*;
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
            size: (3000, 1500),
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

#[derive(Clone)]
struct SizedFault<'a> {
    fault: &'a Fault,
    array: Array<f64, Dim<[usize; 1]>>,
}

fn map_color(i: usize) -> image::Rgb<u8> {
    image::Rgb([RED[i], GREEN[i], BLUE[i]])
}

pub fn map_image(data: &MapData) -> Result<egui::ColorImage, ImageError> {
    let imgx = data.size.0 as f64;
    let imgy = data.size.1 as f64;

    let y_range_div_2 = (imgy as f64) / 2.;
    let y_range_div_pi = (imgy as f64) / PI;
    let mut world_heights = Array::zeros((imgx as usize, imgy as usize));
    let sin_iter_phi = |x: f64| (x * 2. * PI / imgx).sin();

    let sized_faults: Vec<SizedFault> = data
        .faults
        .par_iter()
        .map(|w| SizedFault {
            fault: w,
            array: Array::from_shape_fn([data.size.0 as usize], |i| {
                let sin_iter_index = imgx * (w.xsi + 1.) - i as f64;
                let atan_args = sin_iter_phi(sin_iter_index) * w.tan_b;
                y_range_div_pi * atan_args.atan() + y_range_div_2
            }),
        })
        .collect();

    println!("Processed Sized Faults");

    world_heights
        .indexed_iter_mut()
        .for_each(|(coords, height)| {
            let total_height: isize = sized_faults
                .par_iter()
                .map(|w| {
                    // let sin_iter_index = imgx * (w.xsi + 1.) - coords.0 as f64;
                    // let atan_args = sin_iter_phi(sin_iter_index) * w.tan_b;
                    // let theta = y_range_div_pi * atan_args.atan() + y_range_div_2;
                    let a = w.fault.flag;
                    let b = coords.1 as f64 <= w.array[coords.0];
                    if !(a && b) && (a || b) {
                        1
                    } else {
                        -1
                    }
                })
                .sum();
            *height = total_height;
        });
    println!("Finished calculating world heights");

    let world_max = *world_heights.iter().max().unwrap() as f64;
    let world_min = *world_heights.iter().min().unwrap() as f64;
    let world_range = world_max - world_min;
    let mut imgbuf = image::ImageBuffer::new(data.size.0, data.size.1);
    imgbuf.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let i = (world_heights[[x as usize, y as usize]] as f64 - world_min) / world_range * 47.;
        *pixel = map_color(i as usize);
    });

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
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
