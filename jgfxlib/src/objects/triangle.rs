// Rectangle aligned with X-Y Plane

use std::{sync::Arc, ops::Range};
use rand::rngs::SmallRng;
use crate::{
    materials::Material, 
    aabb::AABB, 
    point3::Point3, 
    ray::Ray, vec3::Vec3,
    objects::{Object, Intersection}, utils::{fmin, fmax}
};

use super::AuxObjectData;

pub struct Triangle {
    material: Arc<dyn Material>,
    p0: Point3,
    p1: Point3,
    p2: Point3,
    n: Vec3,
    uv: Option<(f64,f64)>
}

/// Create triangle defined by corners P0, P1, P2, with given normal vector (optional)
pub fn new(p0: Point3, p1: Point3, p2: Point3, n: Option<Vec3>, uv: Option<(f64,f64)>, material: Arc<dyn Material>) -> Object {
    let n = match n {
        Some(n) => n,
        None => (&p1-&p0).cross(&(&p2-&p1)),
    };
    
    let data = Triangle {
        p0, p1, p2, n, uv,
        material: material.clone()
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::Triangle(data)
    }
}

/// Create canonical triangle on X-Y plane defined by corners P0(0.0, 0.0, 0.0), P1(0.0, 1.0, 0.0), P2(1.0, 0.0, 0.0) 
pub fn canonical(material: Arc<dyn Material>) -> Object {
    let p0 = Point3::zero();
    let p1 = Point3::new(0.0,1.0,0.0);
    let p2 = Point3::new(1.0,0.0,0.0);
    let n = (&p1-&p0).cross(&(&p2-&p1));

    new(p0, p1, p2, Some(n), None, material)
}


fn bounding_box(obj: &Object, _: Range<f64>) -> Option<AABB> {
    let aux = if let AuxObjectData::Triangle(aux) = &obj.aux { aux } else { panic!("Could not extract Triangle from aux data") };

    // The bounding box must have non-zero width in each dimension, so pad the Z
    // dimension a small amount
    let x_min = fmin(fmin(aux.p0.x, aux.p1.x), aux.p2.x);
    let y_min = fmin(fmin(aux.p0.y, aux.p1.y), aux.p2.y);
    let z_min = fmin(fmin(aux.p0.z, aux.p1.z), aux.p2.z);

    let x_max = fmax(fmax(aux.p0.x, aux.p1.x), aux.p2.x);
    let y_max = fmax(fmax(aux.p0.y, aux.p1.y), aux.p2.y);
    let z_max = fmax(fmax(aux.p0.z, aux.p1.z), aux.p2.z);

    let tolerance = Vec3::from_value(0.0001);

    Some(AABB::new(
        Point3::new(x_min, y_min, z_min) - &tolerance, 
        Point3::new(x_max, y_max, z_max) + &tolerance
    ))
}

fn intersect(obj: &Object, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::Triangle(aux) = &obj.aux { aux } else { panic!("Could not extract Triangle from aux data") };

    let t = (&aux.p0 - &r.origin).dot(&aux.n)/(r.dir.dot(&aux.n));

    if t < t_min || t > t_max {
        return None;
    }

    let p = r.at(t);
    let n = aux.n;

    let uv = match aux.uv {
        Some(uv) => uv,
        None => {
            // compute uv
            todo!()
        }
    };

    let v = &p - &aux.p0;

    // check side 1
    let e01 = &aux.p1 - &aux.p0;
    if e01.cross(&v).dot(&n) < 0.0 {
        return None;
    }

    // check side 2
    let v = &p - &aux.p1;
    let e12 = &aux.p2 - &aux.p1;
    if e12.cross(&v).dot(&n) < 0.0 {
        return None;
    }

    // check side 3
    let v = &p - &aux.p2;
    let e20 = &aux.p0 - &aux.p2;
    if e20.cross(&v).dot(&n) < 0.0 {
        return None;
    }

    let mut rec = Intersection::new(t, p, n, &aux.material, uv.0, uv.1);
    rec.set_face_normal(r);
    
    Some(rec)
}
