use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closed_so_far = ray_t.max;

        for hittable in self {
            if let Some(rec) = hittable.hit(r, Interval::new(ray_t.min, closed_so_far)) {
                closed_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
