extern crate nalgebra as na;
use na::{Point3, Vector3};

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

pub struct HitRecord<'a> {
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    /// Sets the hit record's normal vector based on the given ray and outward normal.
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: std::marker::Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
