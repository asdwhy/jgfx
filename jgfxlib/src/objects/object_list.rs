use std::ops::{Range};
use rand::rngs::SmallRng;
use crate::{
    aabb::{AABB, surrounding_box},
    ray::Ray,
    objects::{Intersection, Object, AuxObjectData}
};

pub struct ObjectList {
    pub objects: Vec<Object>
}

/// Create object that is composed of multiple objects. Implemented as a list.
pub fn new() -> Object {
    let data = ObjectList {
        objects: vec![]
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::ObjectList(data)
    }
}

/// Add object to this object list
pub fn add(obj: &mut Object, object: Object) {
    let aux = if let AuxObjectData::ObjectList(aux) = &mut obj.aux { aux } else { panic!("Could not extract ObjectList from aux data") };

    aux.objects.push(object);
}

/// Clear this object list
pub fn clear(obj: &mut Object) {
    let aux = if let AuxObjectData::ObjectList(aux) = &mut obj.aux { aux } else { panic!("Could not extract ObjectList from aux data") };

    aux.objects.clear();
}


fn bounding_box(obj: &Object, time: Range<f64>) -> Option<AABB> {
    let aux = if let AuxObjectData::ObjectList(aux) = &obj.aux { aux } else { panic!("Could not extract ObjectList from aux data") };

    if aux.objects.is_empty() {
        return None
    }

    let mut output_box: Option<AABB> = None;

    for obj in &aux.objects {
        let the_box = match (obj.bounding_box)(&obj, time.clone()) {
            None => return None,
            Some(the_box) => the_box
        };

        output_box = match output_box {
            Some(current_box) => Some(surrounding_box(current_box, the_box)),
            None => Some(the_box),
        };
    }

    output_box
}

/// Computes intersection of given ray with canonical list
fn intersect(obj: &Object, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::ObjectList(aux) = &obj.aux { aux } else { panic!("Could not extract ObjectList from aux data") };
     
    let mut ret: Option<Intersection> = None;
    let mut closest_t = t_max;

    for obj in aux.objects.iter() {
        if let Some(rec) = (obj.intersect)(&obj, rng, r, t_min, closest_t) {
            closest_t = rec.t;
            ret = Some(rec);
        }
    }

    ret
}
