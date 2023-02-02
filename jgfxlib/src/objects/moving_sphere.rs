use std::{f64::consts::PI, ops::Range, sync::Arc};
use rand::rngs::SmallRng;
use crate::{
    aabb::{AABB, surrounding_box},
    point3::Point3,
    materials::Material,
    ray::Ray,
    utils::in_range,
    objects::{Object, AuxObjectData, Intersection},
    vec3::Vec3
};

pub struct MovingSphere {
    pub origin0: Point3,
    pub origin1: Point3,
    pub time: Range<f64>,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

/// Create movable sphere defined by starting and ending origins
pub fn new(
    origin0: Point3, 
    origin1: Point3, 
    time: Range<f64>,
    radius: f64,
    material: Arc<dyn Material>
) -> Object {
    let data = MovingSphere {
        origin0, origin1, time, radius,
        material: material.clone()
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::MovingSphere(data)
    }
}

fn get_origin(aux: &MovingSphere, time: f64) -> Point3 {
    &aux.origin0 + ((time - aux.time.start) / (aux.time.end - aux.time.start))*(&aux.origin1 - &aux.origin0)
}

fn bounding_box(obj: &Object, time: Range<f64>) -> Option<AABB> {
    let aux = if let AuxObjectData::MovingSphere(aux) = &obj.aux { aux } else { panic!("Could not extract MovingSphere from aux data") };

    let box0 = AABB::new(
        get_origin(&aux, time.start) - Vec3::new(aux.radius, aux.radius, aux.radius),
        get_origin(&aux, time.start) + Vec3::new(aux.radius, aux.radius, aux.radius)
    );

    let box1 = AABB::new(
        get_origin(&aux, time.end) - Vec3::new(aux.radius, aux.radius, aux.radius),
        get_origin(&aux, time.end) + Vec3::new(aux.radius, aux.radius, aux.radius)
    );

    Some(surrounding_box(box0, box1))
}

fn intersect(obj: &Object, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::MovingSphere(aux) = &obj.aux { aux } else { panic!("Could not extract MovingSphere from aux data") };

    let oc = &r.origin - get_origin(&aux, r.time);
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

    let t = root;
    let p = r.at(root);
    let n = (&p - get_origin(&aux, r.time)) / aux.radius;
    let uv = get_sphere_uv(&n);

    let mut rec = Intersection::new(t, p, n, &aux.material, uv.0, uv.1);
    rec.set_face_normal(r);

    Some(rec)
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