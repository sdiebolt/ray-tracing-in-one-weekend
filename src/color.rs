extern crate nalgebra as na;
use na::Vector3;

pub type Color = Vector3<f64>;

/// Format a `Color` as an RGB string.
pub fn write_color(color: &Color) {
    println!(
        "{} {} {}",
        (256.0 * color[0].clamp(0.0, 0.999)) as u8,
        (256.0 * color[1].clamp(0.0, 0.999)) as u8,
        (256.0 * color[2].clamp(0.0, 0.999)) as u8
    )
}
