// Rectangle aligned with Y-Z Plane

use std::{sync::Arc, ops::Range};
use rand::rngs::SmallRng;
use crate::{
    materials::Material, 
    aabb::AABB, 
    point3::Point3, 
    ray::Ray, 
    vec3::Vec3,
    objects::{Object, Intersection}
};

pub struct YzRectangle {
    material: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    x: f64
}

impl YzRectangle {
    /// Create rectangle defined by corners P0(x, y0, z0), P1(x, y1, z1)
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, x: f64, material: Arc<dyn Material>) -> Self {
        Self {
            y0, y1, z0, z1, x,
            material: material.clone()
        }
    }

    /// Create canonical rectangle on Y-Z plane defined by corners P0(0.0, 0.0, 0.0), P1(0.0, 1.0, 1.0) 
    pub fn canonical(material: Arc<dyn Material>) -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0, 0.0, material)
    }
}

impl Object for YzRectangle {
    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount

        Some(AABB::new(
            Point3::new(self.x-0.0001, self.y0, self.z0), 
            Point3::new(self.x+0.0001, self.y1, self.z1)
        ))
    }

    fn intersect(&self, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let t = (self.x - r.origin.x) / r.dir.x;

        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin.y + t * r.dir.y;
        let z = r.origin.z + t * r.dir.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let uv = ((y-self.y0)/(self.y1-self.y0), (z-self.z0)/(self.z1-self.z0));

        let outward_normal = Vec3::new(1.0,0.0,0.0);
        let p = r.at(t);

        let mut rec = Intersection::new(t, p, outward_normal, &self.material, uv.0, uv.1);
        rec.set_face_normal(r);
        
        Some(rec)
    }
}