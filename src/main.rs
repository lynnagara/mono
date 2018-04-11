mod image;

// use image::Image;


fn main() {
    let img_path = "./images/cat.jpg";
    let out_path = "./images/out.png";

    image::Image::load(img_path).pixelate(10).resize(500, 500).save(out_path);

}
