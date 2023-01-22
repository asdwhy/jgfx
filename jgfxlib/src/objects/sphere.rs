use std::{f64::consts::{PI, TAU}, ops::Range, sync::Arc};
use rand::rngs::SmallRng;
use crate::{
    aabb::AABB,
    materials::Material,
    point3::Point3,
    ray::Ray,
    utils::in_range,
    objects::{Intersection, Object}, 
};

pub struct Sphere {
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(material: Arc<dyn Material>) -> Self where Self: Sized {
        Self {
            material: material.clone(),
        }
    }

    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = p.y.atan2(p.x);
        let phi = (p.z / p.length()).acos();
    
        let u = (theta+PI)/TAU;
        let v = phi / PI;
    
        (u, v)
    }
}

impl Object for Sphere {
    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        Some(AABB::new(Point3::from_value(-1.0), Point3::from_value(1.0)))
    }

    fn intersect(&self, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let oc = &r.origin;
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.length_squared() - 1.0;

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

        // hit record information with canonical sphere
        let t = root;
        let p = r.at(root);
        let uv = Self::get_sphere_uv(&p);
        let n = 2.0 * p;
        
        let mut rec = Intersection::new(t, p, n, &self.material, uv.0, uv.1);

        rec.set_face_normal(&r);
        
        Some(rec)
    }
}