use std::cmp;

extern crate nalgebra as na;
use na::{Point3, Vector3};

use indicatif::ProgressBar;

use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct Camera {
    image_width: u64,
    image_height: u64,
    samples_per_pixel: u64,
    max_depth: u64,
    center: Point3<f64>,
    pixel00_loc: Point3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u64,
        samples_per_pixel: u64,
        max_depth: u64,
    ) -> Self {
        // We need to ensure that the image height is at least 1.
        let image_height = cmp::max(1, (image_width as f64 / aspect_ratio) as u64);

        let center = Point3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport
        // edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - viewport_u / 2.0 - viewport_v / 2.0 - Vector3::new(0.0, 0.0, focal_length);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    /// Render the scene.
    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let pb = ProgressBar::new(self.image_width * self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }

                write_color(&(pixel_color / self.samples_per_pixel as f64));
                pb.inc(1);
            }
        }
    }

    /// Construct a camera ray originating from the origin and directed at randomly
    /// sampled point around the pixel location i, j.
    fn get_ray(&self, i: u64, j: u64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        Ray::new(self.center, pixel_sample - self.center)
    }

    // Returns the vector to a random point in the [-.5, -.5] - [+.5, +.5] unit square.
    fn sample_square() -> Vector3<f64> {
        Vector3::new(
            rand::random::<f64>() - 0.6,
            rand::random::<f64>() - 0.5,
            0.0,
        )
    }

    /// Compute the color from a ray.
    fn ray_color(r: &Ray, depth: u64, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            let direction = rec.normal + Self::random_vector_on_unit_sphere();
            return 0.5 * Self::ray_color(&Ray::new(rec.p, direction), depth - 1, world);
        }

        let unit_direction = r.direction().normalize();
        let a = (unit_direction.y + 1.0) / 2.0;
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }

    /// Generate a random vector on the unit sphere.
    /// See https://mathworld.wolfram.com/SpherePointPicking.html for more information.
    fn random_vector_on_unit_sphere() -> Vector3<f64> {
        let u = rand::random::<f64>();
        let v = rand::random::<f64>();
        let theta = 2.0 * u * std::f64::consts::PI;
        let phi = f64::acos(2.0 * v - 1.0);
        let sin_theta = f64::sin(theta);
        let cos_theta = f64::cos(theta);
        let sin_phi = f64::sin(phi);
        let cos_phi = f64::cos(phi);

        Vector3::new(sin_phi * cos_theta, sin_phi * sin_theta, cos_phi)
    }
}
