extern crate nalgebra as na;
use na::Point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::Lambertian;
use sphere::Sphere;

/// Aspect ratio.
const ASPECT_RATIO: f64 = 16.0 / 9.0;

/// Image width. The image height will be calculated based on the aspect ratio.
const IMAGE_WIDTH: u64 = 400;

/// Count of random samples for each pixel.
const SAMPLES_PER_PIXEL: u64 = 100;

/// Maximum depth of the ray bouncing.
const MAX_DEPTH: u64 = 50;

/// Vertical field of view in degrees.
const VFOV: f64 = 90.0;

fn main() {
    // The world contains two spheres: one in the center and one serving as a green
    // ground.
    let mut world = HittableList::new();

    let r = f64::cos(std::f64::consts::PI / 4.0);

    let material_left = Lambertian::new(Color::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(Color::new(1.0, 0.0, 0.0));

    world.push(Box::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        VFOV,
    );
    camera.render(&world);
}
