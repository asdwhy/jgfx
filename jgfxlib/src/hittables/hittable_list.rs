use std::ops::Deref;
use std::sync::Arc;

use crate::ray::Ray;
use crate::{hittables::Hittable};
use crate::hittables::HitRecord;


pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut ret: Option<HitRecord> = None;
        let mut closest_t = t_max;

        for obj in self.objects.iter() {
            if let Some(rec) = obj.deref().intersect(r, t_min, closest_t) {
                closest_t = rec.t;
                ret = Some(rec);
            }
        }

        ret
    }
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

