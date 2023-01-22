use std::ops::{Range};
use std::sync::Arc;
use rand::rngs::SmallRng;
use crate::{
    aabb::{AABB, surrounding_box},
    ray::Ray,
    objects::{Intersection, Object}
};

pub struct ObjectList {
    pub objects: Vec<Arc<dyn Object>>
}

impl ObjectList {
    /// Create hittable object that is composed of multiple hittable objects. Implemented as a list.
    pub fn new() -> Self where Self: Sized {
        Self {
            objects: vec![]
        }
    }

    /// Add object to this hittable list
    pub fn add(&mut self, hittable: Arc<dyn Object>) {
        self.objects.push(hittable.clone());
    }

    /// Clear this hittable list
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Object for ObjectList {
    fn bounding_box(&self, time: Range<f64>) -> Option<AABB> {
        if self.objects.is_empty() {
            return None
        }

        let mut output_box: Option<AABB> = None;

        for object in &self.objects {
            let the_box = match object.bounding_box(time.clone()) {
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
    fn intersect(&self, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {        
        let mut ret: Option<Intersection> = None;
        let mut closest_t = t_max;

        for obj in self.objects.iter() {
            if let Some(rec) = obj.intersect(rng, r, t_min, closest_t) {
                closest_t = rec.t;
                ret = Some(rec);
            }
        }

        ret
    }
}
