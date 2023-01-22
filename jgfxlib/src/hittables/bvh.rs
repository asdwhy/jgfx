use std::{ops::Range, cmp::Ordering};
use std::sync::Arc;
use rand::rngs::SmallRng;
use crate::{
    aabb::{surrounding_box, AABB},
    utils::sort_from,
    hittables::{Hittable, HitRecord, hittable_list::HittableList},
    random::random_i32,
    ray::Ray
};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: AABB
}

impl BvhNode {
    /// Create BVH tree from given HittableList
    pub fn new(list: HittableList, time: Range<f64>) -> Self {
        let mut list = list;
        let len = list.objects.len();
        Self::from_indexes(&mut list.objects, 0, len, time)
    }

    fn from_indexes(src_objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize, time: Range<f64>) -> Self {
        let mut objects = src_objects;

        let axis = random_i32(0..3);

        let comparator = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| -> bool {
            let box_a = a.bounding_box(0.0..0.0); // I know this looks wrong but its right!
            let box_b = b.bounding_box(0.0..0.0); // I think...
            
            if box_a.is_none() && box_b.is_none() {
                eprintln!("No bounding box in BvhNode constructor, a passed hittable had no bounding box implemented");
            }
        
            let box_a = box_a.unwrap();
            let box_b = box_b.unwrap();
        
            box_a.minimum[axis] < box_b.minimum[axis]
        };

        let object_span = end - start;
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

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
            left = Arc::new(Self::from_indexes(objects, start, mid, time.clone()));
            right = Arc::new(Self::from_indexes(objects, mid, end, time.clone()));
        }
        
        let b0 = left.bounding_box(time.clone());
        let b1 = right.bounding_box(time);

        if b0.is_none() || b1.is_none() {
            eprintln!("No bounding box in BvhNode constructor, a passed hittable had no bounding box implemented");
        }

        Self {
            left,
            right,
            bounding_box: surrounding_box(b0.unwrap(), b1.unwrap())
        }     
    }
}


impl Hittable for BvhNode {
    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }

    fn intersect(&self, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.intersect(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.intersect(rng, r, t_min, t_max);

        let hit_right = match &hit_left {
            None => self.right.intersect(rng, r, t_min, t_max),
            Some(rec) => self.right.intersect(rng, r, t_min, rec.t)
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
}
