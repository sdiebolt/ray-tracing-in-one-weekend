extern crate nalgebra as na;
use na::Vector3;

pub type Color = Vector3<f64>;

/// Format a `Color` as an RGB string, using the gamma 2 transform.
pub fn write_color(color: &Color) {
    let gamma = 2.0;
    let scale = 1.0 / gamma;
    let color = color.map(|c| (c.powf(scale).clamp(0.0, 0.999) * 256.0) as u8);
    println!("{} {} {}", color.x, color.y, color.z);
}
