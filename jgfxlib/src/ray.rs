use crate::{point3::Point3, vec3::Vec3};


pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
    pub time: f64
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3, time: f64) -> Self {
        Self {
            origin,
            dir,
            time
        }
    }

    pub fn zero() -> Self {
        Self {
            origin: Point3::zero(),
            dir: Vec3::zero(),
            time: 0.0
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + t * &self.dir
    }
}