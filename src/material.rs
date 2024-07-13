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

#[inline]
fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = (-uv.dot(n)).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = reflect(&r_in.direction(), &rec.normal);
        reflected = reflected.normalize() + self.fuzz * random_vector_on_unit_sphere();

        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive
    /// index over the refractive index of the enclosing media.
    refaction_index: f64,
}

impl Dielectric {
    pub fn new(refaction_index: f64) -> Dielectric {
        Dielectric { refaction_index }
    }

    /// Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.refaction_index
        } else {
            self.refaction_index
        };

        let unit_direction = r_in.direction().normalize();
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rand::random() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction);

        Some((scattered, attenuation))
    }
}
