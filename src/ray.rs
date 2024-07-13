#![allow(dead_code)]
extern crate nalgebra as na;
use na::{Point3, Vector3};

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3<f64> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction * t
    }
}
