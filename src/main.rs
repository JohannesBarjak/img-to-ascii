use std::env;

use image::imageops::FilterType;
use image::GenericImageView;

fn main() {
    let ascii_chars = b" .:-_+|{}()\\/[]xnuvXYULO0Zmwoa%#&8@MW";

    let img_path = env::args().nth(1).unwrap();
    let img = image::open(img_path).unwrap();

    let termsize::Size { rows, cols } = termsize::get().unwrap();
    let img = img.resize(cols as u32 / 3, rows as u32, FilterType::Lanczos3);

    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness =
                0.21 * pixel[0] as f64 + 0.72 * pixel[1] as f64 + 0.07 * pixel[2] as f64;

            let ascii_pixel =
                ascii_chars[((brightness / 255_f64) * (ascii_chars.len() as f64 - 1_f64)) as usize];

            for _ in 0..3 {
                print!("{}", ascii_pixel as char);
            }
        }

        println!();
    }
}
