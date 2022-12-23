use std::sync::Arc;

use crate::materials::Material;
use crate::vec3::Vec3;
use crate::{point3::Point3};
use crate::ray::Ray;
use crate::utils::in_range;
use crate::hittables::{HitRecord,Hittable};

pub struct Sphere {
    pub origin: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(material: Arc<dyn Material>) -> Self where Self: Sized {
        Self {
            origin: Vec3::zero(),
            radius: 1.0,
            material: material.clone()
        }
    }
    
    pub fn set_origin(&mut self, origin: Point3) {
        self.origin = origin;
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
}

impl Hittable for Sphere {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &r.origin - &self.origin;
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let disc = half_b*half_b - a * c;

        if disc < 0.0 { return None }

        let sqrtdisc = disc.sqrt();
        
        // find nearest root in acceptable range
        let mut root = (-half_b - sqrtdisc) / a;

        if !in_range(root, t_min, t_max) {
            root = (-half_b + sqrtdisc) / a;

            if !in_range(root, t_min, t_max) {
                return None
            }
        }

        let t = root;
        let p = r.at(root);
        let n = (&p - &self.origin) / self.radius;

        let mut rec = HitRecord::new(t, p, n, &self.material);
        rec.set_face_normal(r);

        Some(rec)
    }

}
