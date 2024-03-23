use crate::{
    point3::Point3, 
    vec3::Vec3
};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
    pub time: f64,
    pub inv: Vec3 // inverse of ray direction components
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3, time: f64) -> Self {
        Self {
            origin,
            dir,
            time,
            inv: Vec3::new( // precomputed for later use
                1.0/dir.x,
                1.0/dir.y,
                1.0/dir.z
            )
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + t * &self.dir
    }
}