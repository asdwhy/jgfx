use std::{f64::consts::{PI, TAU}, ops::Range, sync::Arc};
use rand::rngs::SmallRng;
use crate::{
    aabb::AABB,
    materials::Material,
    point3::Point3,
    ray::Ray,
    utils::in_range,
    hittables::{HitRecord, Hittable}, 
    affine::Affine
};

pub struct Sphere {
    pub material: Arc<dyn Material>,
    pub transform: Affine
}

impl Sphere {
    pub fn new(material: Arc<dyn Material>) -> Self where Self: Sized {
        Self {
            material: material.clone(),
            transform: Affine::new()
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

impl Hittable for Sphere {
    /// Computes intersection of given ray with canonical sphere
    fn canonical_intersect(&self, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        
        let mut rec = HitRecord::new(t, p, n, &self.material, uv.0, uv.1);

        rec.set_face_normal(&r);
        
        Some(rec)
    }

    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        let (c0, c1) = if self.transform.is_identity() {
            (
                Point3::from_value(-1.0), 
                Point3::from_value(1.0)
            )
        } else {
            (
                self.transform.point_transform(&Point3::from_value(-1.0)), 
                self.transform.point_transform(&Point3::from_value(1.0))
            )
        };

        Some(AABB::new(c0, c1))
    }

    fn get_transformation(&self) -> &Affine {
        &self.transform
    }
}