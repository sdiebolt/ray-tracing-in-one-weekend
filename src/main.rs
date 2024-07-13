extern crate nalgebra as na;
use na::Point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;

use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;

/// Aspect ratio.
const ASPECT_RATIO: f64 = 16.0 / 9.0;

/// Image width. The image height will be calculated based on the aspect ratio.
const IMAGE_WIDTH: u64 = 400;

/// Count of random samples for each pixel.
const SAMPLES_PER_PIXEL: u64 = 100;

fn main() {
    // The world contains two spheres: one in the center and one serving as a green
    // ground.
    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, SAMPLES_PER_PIXEL);
    camera.render(&world);
}
