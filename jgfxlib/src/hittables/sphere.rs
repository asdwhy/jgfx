use std::f64::consts::PI;
use std::ops::Range;
use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::aabb::AABB;
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
    pub fn new(origin: Point3, radius: f64, material: Arc<dyn Material>) -> Self where Self: Sized {
        Self {
            origin, radius,
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
    fn intersect(&self, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        let uv = get_sphere_uv(&n);

        let mut rec = HitRecord::new(t, p, n, &self.material, uv.0, uv.1);
        rec.set_face_normal(r);

        Some(rec)
    }

    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        Some(
            AABB::new(
                &self.origin - Vec3::new(self.radius, self.radius, self.radius),
                &self.origin + Vec3::new(self.radius, self.radius, self.radius)
            )
        )
    }

}


/// p: given a point on the unit sphere
/// returns (u,v): texture coordinates
/// u: returned value in [0,1] of angle around the Y axis from x=-1
/// v: returned value in [0,1] of angle from y=-1 to y=+1
///     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
///     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
///     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    // uses spherical coordinates where theta is angle up from -y axis in 0..PI
    // and phi is angle around Y axis (from -X to +Z to +X to -Z to -X)
    
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + PI;

    (phi / (2.0*PI), theta / PI)
}