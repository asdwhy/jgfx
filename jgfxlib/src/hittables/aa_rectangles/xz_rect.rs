// Rectangle aligned with X-Z Plane

use std::f64::INFINITY;
use std::{sync::Arc, ops::Range};

use rand::Rng;
use rand::rngs::SmallRng;

use crate::constants::EPSILON;
use crate::{materials::Material, aabb::AABB, point3::Point3, ray::Ray, vec3::Vec3};

use crate::hittables::{Hittable, HitRecord};

pub struct XzRectangle {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64
}

impl XzRectangle {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            x0, x1, z0, z1, k,
            material: material.clone()
        }
    }
}

impl Hittable for XzRectangle {
    fn intersect(&self, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.dir.y;

        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.dir.x;
        let z = r.origin.z + t * r.dir.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let uv = ((x-self.x0)/(self.x1-self.x0), (z-self.z0)/(self.z1-self.z0));

        let outward_normal = Vec3::new(0.0,1.0,0.0);
        let p = r.at(t);

        let mut rec = HitRecord::new(t, p, outward_normal, &self.material, uv.0, uv.1);
        rec.set_face_normal(r);
        
        Some(rec)
    }

    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount

        Some(AABB::new(
            Point3::new(self.x0, self.k-0.0001, self.z0), 
            Point3::new(self.x1, self.k+0.0001, self.z1)
        ))
    }

    fn pdf_value(&self, rng: &mut SmallRng, o: &Point3, v: &Vec3) -> f64 {
        let r = Ray::new(*o, *v, 0.0);

        match self.intersect(rng, &r, EPSILON, INFINITY) {
            Some(rec) => {
                let area = (self.x1-self.x0)*(self.z1-self.z0);
                let distance_squared = rec.t * rec.t * v.length_squared();
                let cosine = v.dot(&rec.n).abs() / v.length();

                return distance_squared / (cosine * area)
            },
            None => return 0.0
        }
    }

    fn random(&self, rng: &mut SmallRng, o: &Point3) -> Vec3 {
        let random_point = Point3::new(rng.gen_range(self.x0..self.x1), self.k, rng.gen_range(self.z0..self.z1));

        random_point - o
    }
}