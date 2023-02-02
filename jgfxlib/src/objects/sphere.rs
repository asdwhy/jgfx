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

use super::AuxObjectData;

pub struct Sphere {
    pub material: Arc<dyn Material>,
    origin: Point3,
    radius: f64
}

/// Create sphere object centered at origin with given radius
pub fn new(origin: Point3, radius: f64, material: Arc<dyn Material>) -> Object {
    let data = Sphere {
        material: material.clone(),
        origin, radius
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::Sphere(data)
    }
}

// Create canonical sphere centered at the origin with radius 1
pub fn canonical(material: Arc<dyn Material>) -> Object {
    new(Point3::zero(), 1.0, material)
}

fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    let theta = p.y.atan2(p.x);
    let phi = (p.z / p.length()).acos();

    let u = (theta+PI)/TAU;
    let v = phi / PI;

    (u, v)
}


fn bounding_box(obj: &Object, _: Range<f64>) -> Option<AABB> {
    let aux = if let AuxObjectData::Sphere(aux) = &obj.aux { aux } else { panic!("Could not extract Sphere from aux data") };

    Some(AABB::new(&aux.origin - Point3::from_value(aux.radius), &aux.origin + Point3::from_value(aux.radius)))
}

fn intersect(obj: &Object, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::Sphere(aux) = &obj.aux { aux } else { panic!("Could not extract Sphere from aux data") };

    let oc = &r.origin - &aux.origin;
    let a = r.dir.length_squared();
    let half_b = oc.dot(&r.dir);
    let c = oc.length_squared() - aux.radius * aux.radius;

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
    let uv = get_sphere_uv(&p);
    // let n = 2.0 * p;
    let n = (&p - &aux.origin) / aux.radius;
    
    let mut rec = Intersection::new(t, p, n, &aux.material, uv.0, uv.1);

    rec.set_face_normal(&r);
    
    Some(rec)
}
