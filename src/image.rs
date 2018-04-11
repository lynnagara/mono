extern crate image;

use std::path::Path;
use std::fs::File;
use std::cmp::min;

use image::image::{FilterType, GenericImage, DynamicImage, Pixel};

pub struct Image {
    path: String,
    img: DynamicImage
}


impl Image {
    pub fn load(path: &str) -> Image {
        Image{path: path.to_string(), img: get_image(&path)}
    }

    pub fn save(self, out_path: &str) -> Self {
        save_file(&self.img, &out_path);
        self
    }

    pub fn resize(self, height: u32, width: u32) -> Self {
        let img = resize(self.img, height, width);
        Image {path: self.path, img}
    }

    pub fn pixelate(self, pixel_size: u32) -> Self {
        let img = pixelate_rgba(self.img, pixel_size);
        Image {path: self.path, img}
    }
}


fn resize(img: DynamicImage, height: u32, width: u32) -> DynamicImage {
    img.resize(height, width, FilterType::Gaussian)
}

fn pixelate_rgba(mut img: DynamicImage, pixel_size: u32) -> DynamicImage {
    let (width, height) = img.dimensions();

    let x_steps = width / pixel_size;
    let y_steps = height / pixel_size;


    for x in 0..x_steps {
        for y in 0..y_steps {
            let pixel_count = pixel_size * pixel_size;
            let mut rgba = vec![0; 4];

            for xs in 0..pixel_size {
                for ys in 0..pixel_size {
                    let pixel_data : Vec<_> = img.get_pixel(x * pixel_size + xs, y * pixel_size + ys)
                        .data.iter().map(|channel| *channel as u32).collect();
                    rgba = add_vectors(rgba, pixel_data);
                }
            }

            let avg: Vec<_> = rgba.iter().map(|&channel| channel / pixel_count).collect();


            let pixel =  Pixel::from_channels(avg[0] as u8, avg[1] as u8, avg[2] as u8, avg[3] as u8);
            for xs in 0..pixel_size {
                for ys in 0..pixel_size {
                    img.put_pixel(x * pixel_size + xs, y * pixel_size + ys, pixel);
                 }
            }
        }
    }
    img
}

fn add_vectors(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    let len = min(a.len(), b.len());
    let mut vec = vec![];
    for i in 0..len {
        vec.insert(i, a[i] + b[i]);
    }
    vec
}


fn get_image(path: &str) -> DynamicImage {
    image::open(&Path::new(&path)).expect("Could not open image")
}


fn save_file(image: &DynamicImage, path: &str) {
    let mut out = File::create(&Path::new(&path)).unwrap();
    &image.save(&mut out, image::PNG).expect("Could not be saved");
    println!("Image saved!");
}

