// Rectangle aligned with X-Y Plane

use std::{sync::Arc, ops::Range};
use rand::rngs::SmallRng;
use crate::{
    materials::Material, 
    aabb::AABB, 
    point3::Point3, 
    ray::Ray, vec3::Vec3,
    hittables::{Hittable, HitRecord}, affine::Affine
};

pub struct XyRectangle {
    material: Arc<dyn Material>,
    transform: Affine,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64
}

impl XyRectangle {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            x0, x1, y0, y1, k,
            material: material.clone(),
            transform: Affine::new()
        }
    }
}

impl Hittable for XyRectangle {
    fn canonical_intersect(&self, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.dir.z;

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

        let mut rec = HitRecord::new(t, p, outward_normal, &self.material, uv.0, uv.1);
        rec.set_face_normal(r);
        
        Some(rec)
    }

    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount

        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.k-0.0001), 
            Point3::new(self.x1, self.y1, self.k+0.0001)
        ))
    }

    fn get_transformation(&self) -> &Affine {
        &self.transform
    }
}