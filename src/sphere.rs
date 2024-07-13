use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
extern crate nalgebra as na;
use na::{Point3, Vector3};

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().norm_squared();
        let h = r.direction().dot(&oc);
        let c = oc.norm_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - discriminant_sqrt) / a;
        if !ray_t.surrounds(root) {
            root = (h + discriminant_sqrt) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vector3::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        rec.set_face_normal(r, &((rec.p - self.center) / self.radius));

        Some(rec)
    }
}
