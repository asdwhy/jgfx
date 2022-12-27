use std::{sync::Arc, ops::Range};

use rand::rngs::SmallRng;

use crate::{hittables::Hittable, vec3::Vec3, ray::Ray, aabb::AABB};

use super::HitRecord;


pub struct Translate {
    obj: Arc<dyn Hittable>,
    offset: Vec3
}

impl Translate {
    pub fn new(obj: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            obj: obj.clone(),
            offset: displacement
        }
    }
}

impl Hittable for Translate {
    fn intersect(&self, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let translated_r = Ray::new(&r.origin - &self.offset, r.dir, r.time);

        match self.obj.intersect(rng, &translated_r, t_min, t_max) {
            Some(mut rec) => {
                rec.p += &self.offset;
                rec.set_face_normal(&translated_r);

                Some(rec)
            },
            None => None,
        }
    }

    fn bounding_box(&self, time: Range<f64>) -> Option<AABB> {
        match self.obj.bounding_box(time) {
            Some(the_box) => Some(AABB::new(
                the_box.minimum + &self.offset, 
                the_box.maximum + &self.offset
            )),
            None => None,
        }
    }
}