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
const IMAGE_WIDTH: u64 = 1200;

/// Count of random samples for each pixel.
const SAMPLES_PER_PIXEL: u64 = 500;

/// Maximum depth of the ray bouncing.
const MAX_DEPTH: u64 = 50;

/// Vertical field of view in degrees.
const VFOV: f64 = 20.0;

/// Point the camera is looking from.
const LOOKFROM: Point3<f64> = Point3::new(13.0, 2.0, 3.0);

/// Point the camera is looking at.
const LOOKAT: Point3<f64> = Point3::new(0.0, 0.0, 0.0);

/// Camera-relative "up" direction.
const VUP: na::Vector3<f64> = na::Vector3::new(0.0, 1.0, 0.0);

/// Variation angle of rays through each pixel.
const DEFOCUS_ANGLE: f64 = 0.6;

/// Distance from the camera lookfrom point to the focal plane.
const FOCUS_DISTANCE: f64 = 10.0;

fn main() {
    // The world contains two spheres: one in the center and one serving as a green
    // ground.
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse.
                    let albedo = Color::new_random().component_mul(&Color::new_random());
                    world.push(Box::new(Sphere::new(center, 0.2, Lambertian::new(albedo))));
                } else if choose_mat < 0.95 {
                    // Metal.
                    let albedo = Color::new(0.5, 0.5, 0.5) + Color::new_random() / 2.0;
                    let fuzz = rand::random::<f64>() * 0.5;
                    world.push(Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz))));
                } else {
                    // Glass.
                    world.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
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
