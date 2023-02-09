// Rectangle aligned with Y-Z Plane

use std::{sync::Arc, ops::Range};
use rand::rngs::SmallRng;
use crate::{
    materials::Material, 
    aabb::AABB, 
    point3::Point3, 
    ray::Ray, 
    vec3::Vec3,
    objects::{Object, Intersection, AuxObjectData}
};

pub struct YzRectangle {
    material: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    x: f64
}

/// Create rectangle defined by corners P0(x, y0, z0), P1(x, y1, z1)
pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, x: f64, material: Arc<dyn Material>) -> Object {
    let data = YzRectangle {
        y0, y1, z0, z1, x,
        material: material.clone()
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::YzRectangle(data)
    }
}

/// Create canonical rectangle on Y-Z plane defined by corners P0(0.0, 0.0, 0.0), P1(0.0, 1.0, 1.0) 
pub fn canonical(material: Arc<dyn Material>) -> Object {
    new(0.0, 1.0, 0.0, 1.0, 0.0, material)
}

fn bounding_box(obj: &Object, _: Range<f64>) -> Option<AABB> {
    // The bounding box must have non-zero width in each dimension, so pad the Z
    // dimension a small amount
    let aux = if let AuxObjectData::YzRectangle(aux) = &obj.aux { aux } else { panic!("Could not extract YzRectangle from aux data") };

    Some(AABB::new(
        Point3::new(aux.x-0.0001, aux.y0, aux.z0), 
        Point3::new(aux.x+0.0001, aux.y1, aux.z1)
    ))
}

fn intersect(obj: &Object, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::YzRectangle(aux) = &obj.aux { aux } else { panic!("Could not extract YzRectangle from aux data") };

    let t = (aux.x - r.origin.x) / r.dir.x;

    if t < t_min || t > t_max {
        return None;
    }

    let y = r.origin.y + t * r.dir.y;
    let z = r.origin.z + t * r.dir.z;

    if y < aux.y0 || y > aux.y1 || z < aux.z0 || z > aux.z1 {
        return None;
    }

    let uv = ((y-aux.y0)/(aux.y1-aux.y0), (z-aux.z0)/(aux.z1-aux.z0));

    let outward_normal = Vec3::new(1.0,0.0,0.0);
    let p = r.at(t);

    let mut rec = Intersection::new(t, p, outward_normal, &aux.material, uv.0, uv.1);
    rec.set_face_normal(r);
    
    Some(rec)
}
