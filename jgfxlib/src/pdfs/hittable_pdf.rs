use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::vec3::Vec3;
use crate::{point3::Point3, hittables::Hittable};

use crate::pdfs::Pdf;


pub struct HittablePdf {
    o: Point3,
    obj: Arc<dyn Hittable>
}

impl HittablePdf {
    pub fn new(obj: Arc<dyn Hittable>, origin: Point3) -> Self {
        Self {
            obj: obj.clone(),
            o: origin
        }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, rng: &mut SmallRng, direction: &Vec3) -> f64 {
        self.obj.pdf_value(rng, &self.o, direction)
    }

    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.obj.random(rng, &self.o)
    }
}