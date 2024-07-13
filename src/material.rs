extern crate nalgebra as na;
use na::Vector3;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

/// Generate a random vector on the unit sphere.
/// See https://mathworld.wolfram.com/SpherePointPicking.html for more information.
#[inline]
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

#[inline]
fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(n) * n
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scattered = rec.normal + random_vector_on_unit_sphere();

        // Catch degenerate scattered rays.
        if scattered.iter().any(|&x| x.abs() < 1e-8) {
            scattered = rec.normal;
        }

        let scattered = Ray::new(rec.p, scattered);

        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&r_in.direction(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);

        Some((scattered, self.albedo))
    }
}
