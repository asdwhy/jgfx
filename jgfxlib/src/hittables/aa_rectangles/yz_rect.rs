// Rectangle aligned with Y-Z Plane

use std::{sync::Arc, ops::Range};

use rand::rngs::SmallRng;

use crate::{materials::Material, aabb::AABB, point3::Point3, ray::Ray, vec3::Vec3};

use crate::hittables::{Hittable, HitRecord};

pub struct YzRectangle {
    material: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64
}

impl YzRectangle {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            y0, y1, z0, z1, k,
            material: material.clone()
        }
    }
}

impl Hittable for YzRectangle {
    fn intersect(&self, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / r.dir.x;

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

        let mut rec = HitRecord::new(t, p, outward_normal, &self.material, uv.0, uv.1);
        rec.set_face_normal(r);
        
        Some(rec)
    }

    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount

        Some(AABB::new(
            Point3::new(self.k-0.0001, self.y0, self.z0), 
            Point3::new(self.k+0.0001, self.y1, self.z1)
        ))
    }
}