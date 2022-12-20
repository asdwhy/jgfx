use std::sync::Arc;

use crate::{ray::Ray, point::Point3, vec3::Vec3, materials::Material};

pub struct HitRecord {
    pub p: Point3,
    pub n: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>
}

impl HitRecord {
    pub fn new(t: f64, p: Point3, n: Vec3, material: &Arc<dyn Material>) -> Self {
        Self {
            t, p, n,
            front_face: false,
            material: material.clone()
        }
    }

    /// Ensure that the normal on this rec points against the ray
    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front_face = r.dir.dot(&self.n) < 0.0;
        self.n = if self.front_face { self.n } else { -self.n }
    }
}

pub trait Hittable: Send + Sync {
    // fn new() -> Self where Self: Sized;
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


pub mod hittable_list;
pub mod objects;