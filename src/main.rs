use indicatif::ProgressBar;

mod color;
use color::{write_color, Color};

const IMAGE_WIDTH: u64 = 256;
const IMAGE_HEIGHT: u64 = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let pb = ProgressBar::new(IMAGE_WIDTH * IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            );

            write_color(&pixel_color);
            pb.inc(1);
        }
    }
}
