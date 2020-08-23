use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

const SIZE: u32 = 500;

struct Version<'a> {
    key: &'a str,
    module: u32,
}

enum Mode {
    NUMBER,
}

const VERSIONS: [Version; 1] = [Version {
    key: "m1",
    module: 11,
}];

const WHITE: Rgba<u8> = Rgba([0, 0, 0, 255]);
const BLACK: Rgba<u8> = Rgba([255, 255, 255, 255]);

fn main() {
    let version = &VERSIONS[0];
    let dot_size = SIZE / version.module;
    let mut img = RgbaImage::new(SIZE, SIZE);
    let mut x = 0;
    let mut y = 0;
    let mut is_black = true;
    for _ in 0..version.module {
        for _ in 0..version.module {
            let rect = Rect::at(x, y).of_size(dot_size, dot_size);
            draw_filled_rect_mut(&mut img, rect, if is_black { BLACK } else { WHITE });
            x += dot_size as i32;
            is_black = !is_black;
        }
        x = 0;
        y += dot_size as i32;
    }
    img.save("gqr.png").unwrap();
}
