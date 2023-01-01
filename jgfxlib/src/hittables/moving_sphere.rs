use std::{f64::consts::PI, ops::Range, sync::Arc};
use rand::rngs::SmallRng;
use crate::{
    aabb::{AABB, surrounding_box},
    point3::Point3,
    materials::Material,
    ray::Ray,
    utils::in_range,
    hittables::{Hittable, HitRecord},
    vec3::Vec3
};

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
            origin0, origin1, time, radius,
            material: material.clone()
        }
    }

    pub fn get_origin(&self, time: f64) -> Point3 {
        &self.origin0 + ((time - self.time.start) / (self.time.end - self.time.start))*(&self.origin1 - &self.origin0)
    }
}

impl Hittable for MovingSphere {
    fn intersect(&self, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        let uv = get_sphere_uv(&n);

        let mut rec = HitRecord::new(t, p, n, &self.material, uv.0, uv.1);
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