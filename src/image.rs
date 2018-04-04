extern crate image;

use std::path::Path;
use std::fs::File;
use image::image::imageops::FilterType;
use image::image::DynamicImage;


pub fn resize(path: &str, out_path: &str, height: u32, width: u32) {
    let img = get_file(&path);
    let resized = img.resize(height, width, FilterType::Gaussian);
    save_file(resized, &out_path);
}

// pub fn pixelate(path: &str, out_path: &str, pixel_size: u32) {
// }

fn get_file(path: &str) -> DynamicImage {
    image::open(&Path::new(&path)).expect("Could not open image")
}

fn save_file(image: DynamicImage, path: &str) {
    let mut out = File::create(&Path::new(&path)).unwrap();
    image.save(&mut out, image::PNG).expect("Could not be saved");
    println!("Image saved!");
}
