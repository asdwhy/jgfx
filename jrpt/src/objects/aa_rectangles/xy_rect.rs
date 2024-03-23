// Rectangle aligned with X-Y Plane

use std::{sync::Arc, ops::Range};
use rand::rngs::SmallRng;
use crate::{
    materials::Material, 
    aabb::AABB, 
    point3::Point3, 
    ray::Ray, vec3::Vec3,
    objects::{Object, Intersection, AuxObjectData}
};

pub struct XyRectangle {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z: f64
}

/// Create rectangle defined by corners P0(x0, y0, z), P1(x1, y1, z) 
pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, z: f64, material: Arc<dyn Material>) -> Object {
    let data = XyRectangle {
        x0, x1, y0, y1, z,
        material: material.clone()
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::XyRectangle(data)
    }
}

/// Create canonical rectangle on X-Y plane defined by corners P0(0.0, 0.0, 0.0), P1(1.0, 1.0, 0.0) 
pub fn canonical(material: Arc<dyn Material>) -> Object {
    new(0.0, 1.0, 0.0, 1.0, 0.0, material)
}

fn bounding_box(obj: &Object, _: Range<f64>) -> Option<AABB> {
    // The bounding box must have non-zero width in each dimension, so pad the Z
    // dimension a small amount
    let aux = if let AuxObjectData::XyRectangle(aux) = &obj.aux { aux } else { panic!("Could not extract XyRectangle from aux data") };

    Some(AABB::new(
        Point3::new(aux.x0, aux.y0, aux.z-0.0001), 
        Point3::new(aux.x1, aux.y1, aux.z+0.0001)
    ))
}

fn intersect(obj: &Object, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::XyRectangle(aux) = &obj.aux { aux } else { panic!("Could not extract XyRectangle from aux data") };

    let t = (aux.z - r.origin.z) / r.dir.z;

    if t < t_min || t > t_max {
        return None;
    }

    let x = r.origin.x + t * r.dir.x;
    let y = r.origin.y + t * r.dir.y;

    if x < aux.x0 || x > aux.x1 || y < aux.y0 || y > aux.y1 {
        return None;
    }

    let uv = ((x-aux.x0)/(aux.x1-aux.x0), (y-aux.y0)/(aux.y1-aux.y0));

    let outward_normal = Vec3::new(0.0,0.0,1.0);
    let p = r.at(t);

    let mut rec = Intersection::new(t, p, outward_normal, &aux.material, uv.0, uv.1);
    rec.set_face_normal(r);
    
    Some(rec)
}
