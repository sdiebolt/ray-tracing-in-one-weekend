use rayon::prelude::*;
use std::cmp;

use console::{style, Emoji};

extern crate nalgebra as na;
use na::{Point3, Vector3};

use indicatif::ProgressBar;

use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;

/// Generate a random vector in the unit disk.
/// See https://mathworld.wolfram.com/DiskPointPicking.html for more information.
#[inline]
fn random_vector_in_unit_disk() -> Vector3<f64> {
    let r = rand::random::<f64>().sqrt();
    let theta = 2.0 * rand::random::<f64>() * std::f64::consts::PI;
    Vector3::new(r * f64::cos(theta), r * f64::sin(theta), 0.0)
}

/// A camera.
///
/// The camera is defined by the following parameters:
/// * `image_width` - The width of the rendered image in pixels.
/// * `image_height` - The height of the rendered image in pixels.
/// * `samples_per_pixel` - The number of samples to take for each pixel.
/// * `max_depth` - The maximum number of ray bounces into scene.
/// * `defocus_angle` - The variation angle of rays through each pixel.
/// * `focus_distance` - The distance from the camera lookfrom point to the focal plane.
/// * `center` - The center of the camera.
/// * `pixel00_loc` - The location of the upper left pixel.
/// * `pixel_delta_u` - The vector from pixel to pixel across the horizontal viewport edge.
/// * `pixel_delta_v` - The vector from pixel to pixel down the vertical viewport edge.
/// * `defocus_disk_u` - The defocus disk horizontal radius.
/// * `defocus_disk_v` - The defocus disk vertical radius.
pub struct Camera {
    image_width: u64,
    image_height: u64,
    samples_per_pixel: u64,
    max_depth: u64,
    defocus_angle: f64,
    center: Point3<f64>,
    pixel00_loc: Point3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    defocus_disk_u: Vector3<f64>,
    defocus_disk_v: Vector3<f64>,
}

impl Camera {
    /// Create a new camera.
    ///
    /// # Arguments
    /// * `aspect_ratio` - The aspect ratio of the image.
    /// * `image_width` - The width of the image in pixels.
    /// * `samples_per_pixel` - The number of samples to take for each pixel.
    /// * `max_depth` - The maximum depth of the ray bouncing.
    /// * `vfov` - The vertical field of view in degrees.
    /// * `lookfrom` - The point the camera is looking from.
    /// * `lookat` - The point the camera is looking at.
    /// * `vup` - The camera-relative "up" direction.
    /// * `defocus_angle` - The variation angle of rays through each pixel.
    /// * `focus_distance` - The distance from the camera lookfrom point to the focal plane.
    pub fn new(
        aspect_ratio: f64,
        image_width: u64,
        samples_per_pixel: u64,
        max_depth: u64,
        vfov: f64,
        lookfrom: Point3<f64>,
        lookat: Point3<f64>,
        vup: Vector3<f64>,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        // We need to ensure that the image height is at least 1.
        let image_height = cmp::max(1, (image_width as f64 / aspect_ratio) as u64);

        let center = lookfrom;

        // Determine viewport dimensions.
        let theta = vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport
        // edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - (focus_distance * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_distance * f64::tan(defocus_angle.to_radians() / 2.0);
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            defocus_angle,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    /// Render the scene.
    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        eprintln!(
            "{} {} Casting rays...",
            style("[1/2]").bold().dim(),
            Emoji("⚡", "")
        );
        let pb = ProgressBar::new(self.image_width * self.image_height * self.samples_per_pixel);
        let image = (0..self.image_height)
            .into_par_iter()
            .flat_map(|j| {
                (0..self.image_width)
                    .map(|i| {
                        (0..self.samples_per_pixel)
                            .map(|_| {
                                let r = self.get_ray(i, j);
                                pb.inc(1);
                                Self::ray_color(&r, self.max_depth, world)
                            })
                            .sum::<Color>()
                            / self.samples_per_pixel as f64
                    })
                    .collect::<Vec<Color>>()
            })
            .collect::<Vec<Color>>();
        pb.finish_and_clear();

        eprintln!(
            "{} {} Creating image file...",
            style("[2/2]").bold().dim(),
            Emoji("📁", "")
        );
        let pb = ProgressBar::new(self.image_width * self.image_height);
        for col in image {
            write_color(&col);
            pb.inc(1);
        }
        pb.finish_and_clear();
    }

    /// Construct a camera ray originating from the defocus disk and directed at a
    /// randomly sampled point around the pixel location i, j.
    fn get_ray(&self, i: u64, j: u64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    // Returns the vector to a random point in the [-.5, -.5] - [+.5, +.5] unit square.
    fn sample_square() -> Vector3<f64> {
        Vector3::new(
            rand::random::<f64>() - 0.6,
            rand::random::<f64>() - 0.5,
            0.0,
        )
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3<f64> {
        let p = random_vector_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    /// Compute the color from a ray.
    fn ray_color(r: &Ray, depth: u64, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
                return attenuation.component_mul(&Self::ray_color(&scattered, depth - 1, world));
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().normalize();
        let a = (unit_direction.y + 1.0) / 2.0;
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }
}
