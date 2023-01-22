// Rectangle aligned with X-Y Plane

use std::{sync::Arc, ops::Range};
use rand::rngs::SmallRng;
use crate::{
    materials::Material, 
    aabb::AABB, 
    point3::Point3, 
    ray::Ray, vec3::Vec3,
    objects::{Object, Intersection}
};

pub struct XyRectangle {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z: f64
}

impl XyRectangle {
    /// Create rectangle defined by corners P0(x0, y0, z), P1(x1, y1, z) 
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, z: f64, material: Arc<dyn Material>) -> Self {
        Self {
            x0, x1, y0, y1, z,
            material: material.clone()
        }
    }

    /// Create canonical rectangle on X-Y plane defined by corners P0(0.0, 0.0, 0.0), P1(1.0, 1.0, 0.0) 
    pub fn canonical(material: Arc<dyn Material>) -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0, 0.0, material)
    }
}

impl Object for XyRectangle {
    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount

        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.z-0.0001), 
            Point3::new(self.x1, self.y1, self.z+0.0001)
        ))
    }

    fn intersect(&self, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let t = (self.z - r.origin.z) / r.dir.z;

        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.dir.x;
        let y = r.origin.y + t * r.dir.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let uv = ((x-self.x0)/(self.x1-self.x0), (y-self.y0)/(self.y1-self.y0));

        let outward_normal = Vec3::new(0.0,0.0,1.0);
        let p = r.at(t);

        let mut rec = Intersection::new(t, p, outward_normal, &self.material, uv.0, uv.1);
        rec.set_face_normal(r);
        
        Some(rec)
    }
}