// Rectangle aligned with X-Z Plane

use std::{sync::Arc, ops::Range};
use rand::rngs::SmallRng;
use crate::{
    materials::Material, 
    aabb::AABB, 
    point3::Point3, 
    ray::Ray, 
    vec3::Vec3,
    objects::{Intersection, Object, AuxObjectData}
};

pub struct XzRectangle {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    y: f64
}

/// Create rectangle defined by corners P0(x0, y, z0), P1(x1, y, z1)
pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, y: f64, material: Arc<dyn Material>) -> Object {
    let data = XzRectangle {
        x0, x1, z0, z1, y,
        material: material.clone()
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::XzRectangle(data)
    }
}

/// Create canonical rectangle on X-Z plane defined by corners P0(0.0, 0.0, 0.0), P1(1.0, 0.0, 1.0) 
pub fn canonical(material: Arc<dyn Material>) -> Object {
    new(0.0, 1.0, 0.0, 1.0, 0.0, material)
}


fn bounding_box(obj: &Object, _: Range<f64>) -> Option<AABB> {
    // The bounding box must have non-zero width in each dimension, so pad the Z
    // dimension a small amount
    let aux = if let AuxObjectData::XzRectangle(aux) = &obj.aux { aux } else { panic!("Could not extract XzRectangle from aux data") };

    Some(AABB::new(
        Point3::new(aux.x0, aux.y-0.0001, aux.z0), 
        Point3::new(aux.x1, aux.y+0.0001, aux.z1)
    ))
}

fn intersect(obj: &Object, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::XzRectangle(aux) = &obj.aux { aux } else { panic!("Could not extract XzRectangle from aux data") };

    let t = (aux.y - r.origin.y) / r.dir.y;

    if t < t_min || t > t_max {
        return None;
    }

    let x = r.origin.x + t * r.dir.x;
    let z = r.origin.z + t * r.dir.z;

    if x < aux.x0 || x > aux.x1 || z < aux.z0 || z > aux.z1 {
        return None;
    }

    let uv = ((x-aux.x0)/(aux.x1-aux.x0), (z-aux.z0)/(aux.z1-aux.z0));

    let outward_normal = Vec3::new(0.0,1.0,0.0);
    let p = r.at(t);

    let mut rec = Intersection::new(t, p, outward_normal, &aux.material, uv.0, uv.1);
    rec.set_face_normal(r);
    
    Some(rec)
}
