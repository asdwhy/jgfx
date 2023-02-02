use std::{sync::Arc, ops::Range, f64::{NEG_INFINITY, INFINITY}};
use rand::{rngs::SmallRng, Rng};
use crate::{
    objects::{Object, AuxObjectData, Intersection},
    materials::{Material, isotropic::Isotropic}, 
    textures::{Texture, solid_colour::SolidColour}, 
    colour::Colour, 
    aabb::AABB, 
    ray::Ray,
    vec3::Vec3
};

pub struct ConstantMedium {
    boundary: Arc<Object>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64
}

/// Create a constant medium with boundary given as passed Object with given density and colour
pub fn new(obj: Object, density: f64, colour: Colour) -> Object {
    from_texture(obj, density, Arc::new(SolidColour::new(colour)))
}

/// Create a constant medium with boundary given as passed Object with given density and colour chosen from a texture
pub fn from_texture(obj: Object, density: f64, texture: Arc<dyn Texture>) -> Object {
    let data = ConstantMedium {
        boundary: Arc::new(obj),
        phase_function: Arc::new(Isotropic::from_texture(texture)),
        neg_inv_density: -1.0/density
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::ConstantMedium(data)
    }
}


fn bounding_box(obj: &Object, time: Range<f64>) -> Option<AABB> {
    let aux = if let AuxObjectData::ConstantMedium(aux) = &obj.aux { aux } else { panic!("Could not extract ConstantMedium from aux data") };

    (aux.boundary.bounding_box)(&aux.boundary, time)
}

fn intersect(obj: &Object, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::ConstantMedium(aux) = &obj.aux { aux } else { panic!("Could not extract ConstantMedium from aux data") };

    let mut rec1 = if let Some(rec) = (aux.boundary.intersect)(&aux.boundary, rng, r, NEG_INFINITY, INFINITY) { rec } else {
        return None;
    };

    let mut rec2 = if let Some(rec) = (aux.boundary.intersect)(&aux.boundary, rng, r, rec1.t + 0.0001, INFINITY) { rec } else {
        return None;
    };

    if rec1.t < t_min { rec1.t = t_min; }
    if rec2.t > t_max { rec2.t = t_max; }

    if rec1.t >= rec2.t {
        return None;
    }

    if rec1.t < 0.0 {
        rec1.t = 0.0;
    }

    let ray_len = r.dir.length();
    let distance_inside_boundary = (rec2.t - rec1.t) * ray_len;
    let hit_distance = aux.neg_inv_density * rng.gen::<f64>().ln();

    if hit_distance > distance_inside_boundary {
        return None;
    }

    let t = rec1.t + hit_distance / ray_len;
    let p = r.at(t);

    let n = Vec3::new(1.0,0.0,0.0);

    let mut rec = Intersection::new(t, p, n, &aux.phase_function, 0.0, 0.0);
    rec.set_face_normal(r); // arbitrary decision

    Some(rec)
}
