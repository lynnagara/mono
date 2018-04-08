mod image;

fn main() {
    let img_path = "./images/cat.jpg";
    let out_path = "./images/out.png";
    image::pixelate_rgba(img_path, out_path, 10);
}
