use std::ops::{Range};
use std::sync::Arc;

use rand::Rng;
use rand::rngs::SmallRng;

use crate::aabb::{AABB, surrounding_box};
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;
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

    pub fn get(&self, i: usize) -> &Arc<dyn Hittable> {
        self.objects.get(i).unwrap()
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

    fn pdf_value(&self, rng: &mut SmallRng, o: &Point3, v: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for obj in self.objects.iter() {
            sum += weight * obj.pdf_value(rng, o, v);
        }

        sum
    }

    fn random(&self, rng: &mut SmallRng, o: &Point3) -> Vec3 {
        let i = rng.gen_range(0..self.objects.len());

        self.objects.get(i).unwrap().random(rng, o)
    }
}
