use std::{sync::Arc, ops::Range};
use rand::rngs::SmallRng;
use crate::{
    ray::Ray, 
    point3::Point3, 
    vec3::Vec3, 
    materials::Material, 
    aabb::AABB
};

pub struct Intersection {
    pub p: Point3,                      // point of intersection
    pub n: Vec3,                        // normal at point of intersection
    pub t: f64,                         // distance ray travelled
    pub front_face: bool,               // did the ray hit the outside
    pub material: Arc<dyn Material>,    // material hit
    pub u: f64, pub v: f64              // texture u-v coordinates
}

impl Intersection {
    pub fn new(t: f64, p: Point3, n: Vec3, material: &Arc<dyn Material>, u: f64, v: f64) -> Self {
        Self {
            t, p, n,
            front_face: false,
            material: material.clone(),
            u, v
        }
    }

    /// Ensure that the normal on this rec points against the ray
    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front_face = r.dir.dot(&self.n) < 0.0;
        
        if !self.front_face { 
            self.n = - &self.n; 
        }
    }
}

pub trait Object: Send + Sync {
    fn bounding_box(&self, time: Range<f64>) -> Option<AABB>;

    fn intersect(&self, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

pub mod object_list;
pub mod sphere;
pub mod moving_sphere;
pub mod bvh;
pub mod aa_rectangles;
pub mod rect_prism;
pub mod constant_medium;
pub mod affine;
pub mod wavefront_obj;
pub mod triangle;