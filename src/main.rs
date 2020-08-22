use image::RgbaImage;

const SIZE: u32 = 500;

enum Version {
    M1,
}

enum Mode {
    NUMBER,
}

fn main() {
    let mut img = RgbaImage::new(SIZE, SIZE);

    img.save("gqr.png").unwrap();
}
