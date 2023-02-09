use std::{ops::Range, cmp::Ordering, sync::Arc};
use rand::rngs::SmallRng;
use crate::{
    aabb::{surrounding_box, AABB},
    utils::sort_from,
    objects::{Object, AuxObjectData, Intersection},
    random::random_i32,
    ray::Ray
};

pub struct BvhNode {
    left: Arc<Object>,
    right: Arc<Object>,
    bounding_box: AABB
}

/// Create BVH tree from given ObjectList
pub fn new(list: Object, time: Range<f64>) -> Object {
    let aux = if let AuxObjectData::ObjectList(aux) = list.aux { aux } else { panic!("Could not extract ObjectList from aux data") };

    if aux.objects.is_empty() {
        panic!("Tried to create BVH tree from empty list");
    }

    let len = aux.objects.len();
    let list: Vec<Arc<Object>> = aux.objects.into_iter().map(Arc::new).collect();

    from_indexes(list, 0, len, time)
}

fn from_indexes(src_objects: Vec<Arc<Object>>, start: usize, end: usize, time: Range<f64>) -> Object {
    let mut objects = src_objects;

    let axis = random_i32(0..3);

    let comparator = |a: &Object, b: &Object| -> bool {
        let box_a = (a.bounding_box)(a, 0.0..0.0); // I know this looks wrong but its right!
        let box_b = (b.bounding_box)(b, 0.0..0.0); // I think...
        
        if box_a.is_none() && box_b.is_none() {
            eprintln!("No bounding box in BvhNode constructor, a passed object had no bounding box implemented");
        }
    
        let box_a = box_a.unwrap();
        let box_b = box_b.unwrap();
    
        box_a.minimum[axis] < box_b.minimum[axis]
    };

    let object_span = end - start;
    let left: Arc<Object>;
    let right: Arc<Object>;

    if object_span == 1 { // base case only one object, copy it to both subtrees
        left = objects[start].clone();
        right = objects[start].clone();
    } else if object_span == 2 { // base case 2 objects, left will be lesser
        if comparator(&objects[start], &objects[start+1]) {
            left = objects[start].clone();
            right = objects[start+1].clone();
        } else {
            left = objects[start+1].clone();
            right = objects[start].clone();
        }
    } else { // recursive case
        sort_from(&mut objects, start, end, |a,b| {
            if comparator(a,b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let mid = start + object_span / 2;
        left = Arc::new(from_indexes(objects.clone(), start, mid, time.clone()));
        right = Arc::new(from_indexes(objects, mid, end, time.clone()));
    }
    
    let b0 = (left.bounding_box)(&left, time.clone());
    let b1 = (right.bounding_box)(&right, time);

    if b0.is_none() || b1.is_none() {
        eprintln!("No bounding box in BvhNode constructor, a passed object had no bounding box implemented");
    }

    let data = BvhNode {
        left,
        right,
        bounding_box: surrounding_box(b0.unwrap(), b1.unwrap())
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::BvhNode(data)
    }
}


fn bounding_box(obj: &Object, _: Range<f64>) -> Option<AABB> {
    let aux = if let AuxObjectData::BvhNode(aux) = &obj.aux { aux } else { panic!("Could not extract BvhNode from aux data") };

    Some(aux.bounding_box.clone())
}

fn intersect(obj: &Object, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::BvhNode(aux) = &obj.aux { aux } else { panic!("Could not extract BvhNode from aux data") };

    if !aux.bounding_box.intersect(r, t_min, t_max) {
        return None;
    }

    let hit_left = (aux.left.intersect)(&aux.left, rng, r, t_min, t_max);

    let hit_right = match &hit_left {
        None => (aux.right.intersect)(&aux.right, rng, r, t_min, t_max),
        Some(rec) => (aux.right.intersect)(&aux.right, rng, r, t_min, rec.t)
    };

    let hl = hit_left.is_some();
    let hr = hit_right.is_some();

    if hl && hr {
        return hit_right;
    } else if hl {
        return hit_left;
    } else if hr {
        return hit_right;
    } else {
        return None;
    }
}

