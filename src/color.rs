extern crate nalgebra as na;
use na::Vector3;

pub type Color = Vector3<f64>;

/// Format a `Color` as an RGB string.
pub fn write_color(color: &Color) {
    println!(
        "{} {} {}",
        (255.999 * color[0]) as u8,
        (255.999 * color[1]) as u8,
        (255.999 * color[2]) as u8
    )
}
