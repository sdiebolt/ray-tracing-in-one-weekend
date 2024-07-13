use std::cmp;

extern crate nalgebra as na;
use na::{Point3, Vector3};

use indicatif::ProgressBar;

mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;

use color::{write_color, Color};
use hittable::Hittable;
use hittable_list::HittableList;
use interval::Interval;
use ray::Ray;
use sphere::Sphere;

/// Aspect ratio.
const ASPECT_RATIO: f64 = 16.0 / 9.0;

/// Image width. The image height will be calculated based on the aspect ratio.
const IMAGE_WIDTH: u64 = 400;

/// Compute the color from a ray.
fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = r.direction().normalize();
    let a = (unit_direction.y + 1.0) / 2.0;
    return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
}

fn main() {
    // We need to ensure that the image height is at least 1.
    let image_height: u64 = cmp::max(1, (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64);

    // Camera parameters and viewport size.
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (IMAGE_WIDTH as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // The world contains two spheres: one in the center and one serving as a green
    // ground.
    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vector3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, image_height);

    let pb = ProgressBar::new(IMAGE_WIDTH * image_height);
    for j in 0..image_height {
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);

            write_color(&pixel_color);
            pb.inc(1);
        }
    }
}
