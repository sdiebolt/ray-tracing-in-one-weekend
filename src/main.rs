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
use material::{Dielectric, Lambertian, Metal};
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
const VFOV: f64 = 20.0;

/// Point the camera is looking from.
const LOOKFROM: Point3<f64> = Point3::new(-2.0, 2.0, 1.0);

/// Point the camera is looking at.
const LOOKAT: Point3<f64> = Point3::new(0.0, 0.0, -1.0);

/// Camera-relative "up" direction.
const VUP: na::Vector3<f64> = na::Vector3::new(0.0, 1.0, 0.0);

/// Variation angle of rays through each pixel.
const DEFOCUS_ANGLE: f64 = 10.0;

/// Distance from the camera lookfrom point to the focal plane.
const FOCUS_DISTANCE: f64 = 3.4;

fn main() {
    // The world contains two spheres: one in the center and one serving as a green
    // ground.
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        VFOV,
        LOOKFROM,
        LOOKAT,
        VUP,
        DEFOCUS_ANGLE,
        FOCUS_DISTANCE,
    );
    camera.render(&world);
}
