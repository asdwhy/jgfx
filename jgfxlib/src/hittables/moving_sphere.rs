use std::ops::Range;
use std::sync::Arc;

use crate::aabb::{AABB, surrounding_box};
use crate::point3::Point3;
use crate::materials::Material;
use crate::ray::Ray;
use crate::utils::in_range;
use crate::hittables::{Hittable, HitRecord};
use crate::vec3::Vec3;


pub struct MovingSphere {
    pub origin0: Point3,
    pub origin1: Point3,
    pub time: Range<f64>,
    pub radius: f64,
    pub material: Arc<dyn Material>
}

impl MovingSphere {
    pub fn new(
        origin0: Point3, 
        origin1: Point3, 
        time: Range<f64>,
        radius: f64,
        material: Arc<dyn Material>
    ) -> Self {
        Self {
            origin0, origin1,
            time,
            radius, material
        }
    }

    pub fn get_origin(&self, time: f64) -> Point3 {
        &self.origin0 + ((time - self.time.start) / (self.time.end - self.time.start))*(&self.origin1 - &self.origin0)
    }
}

impl Hittable for MovingSphere {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &r.origin - self.get_origin(r.time);
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
        let n = (&p - self.get_origin(r.time)) / self.radius;

        let mut rec = HitRecord::new(t, p, n, &self.material);
        rec.set_face_normal(r);

        Some(rec)
    }

    fn bounding_box(&self, time: Range<f64>) -> Option<AABB> {
        let box0 = AABB::new(
            self.get_origin(time.start) - Vec3::new(self.radius, self.radius, self.radius),
            self.get_origin(time.start) + Vec3::new(self.radius, self.radius, self.radius)
        );

        let box1 = AABB::new(
            self.get_origin(time.end) - Vec3::new(self.radius, self.radius, self.radius),
            self.get_origin(time.end) + Vec3::new(self.radius, self.radius, self.radius)
        );

        Some(surrounding_box(box0, box1))
    }
}