use crate::{point::Point3, vec3::Vec3};


pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3
}

impl Ray {

    pub fn new(orig: &Point3, dir: &Vec3) -> Self {
        Self {
            origin: orig.clone(),
            dir: dir.clone()
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + t * &self.dir
    }
}