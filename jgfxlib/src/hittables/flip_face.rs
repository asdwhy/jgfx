use std::{sync::Arc, ops::Range};

use rand::rngs::SmallRng;

use crate::{hittables::Hittable, ray::Ray, aabb::AABB};

use super::HitRecord;


pub struct FlipFace {
    obj: Arc<dyn Hittable>
}

impl FlipFace {
    pub fn new(obj: Arc<dyn Hittable>) -> Self {
        Self {
            obj: obj.clone()
        }
    }
}

impl Hittable for FlipFace {
    fn intersect(&self, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.obj.intersect(rng, r, t_min, t_max) {
            Some(mut rec) => {
                rec.front_face = !rec.front_face;
                Some(rec)
            },
            None => None
        }
    }

    fn bounding_box(&self, time: Range<f64>) -> Option<AABB> {
        self.obj.bounding_box(time)
    }
}