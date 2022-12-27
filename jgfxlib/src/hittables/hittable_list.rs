use std::ops::{Range};
use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::aabb::{AABB, surrounding_box};
use crate::ray::Ray;
use crate::{hittables::Hittable};
use crate::hittables::HitRecord;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self where Self: Sized {
        Self {
            objects: vec![]
        }
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.objects.push(hittable.clone());
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn intersect(&self, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut ret: Option<HitRecord> = None;
        let mut closest_t = t_max;

        for obj in self.objects.iter() {
            if let Some(rec) = obj.intersect(rng, r, t_min, closest_t) {
                closest_t = rec.t;
                ret = Some(rec);
            }
        }

        ret
    }

    fn bounding_box(&self, time: Range<f64>) -> Option<AABB> {
        if self.objects.is_empty() {
            return None
        }

        let mut output_box: Option<AABB> = None;

        for object in &self.objects {
            match object.bounding_box(time.clone()) {
                None => return None,
                Some(the_box) => {
                    output_box = match output_box {
                        None => Some(the_box),
                        Some(current_box) => Some(surrounding_box(current_box, the_box))
                    };
                }
            }
        }

        output_box
    }
}
