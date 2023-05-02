use image::{imageops, GenericImageView};

fn get_image(dir: &str) {
    let img = image::open(dir).unwrap();
    println!("{:?}", img.dimensions());
    let new_image = imageops::resize(&img, 300, 192, imageops::Lanczos3);
    println!("{:?}", new_image.dimensions());
    new_image.save("./image_rust.jpeg").unwrap();
}

fn main() {
    get_image("../real.jpeg");
}
