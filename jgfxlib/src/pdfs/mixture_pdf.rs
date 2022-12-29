use std::sync::Arc;

use rand::{rngs::SmallRng, Rng};

use crate::{pdfs::Pdf, vec3::Vec3};


pub struct MixturePdf {
    pdf0: Arc<dyn Pdf>,
    pdf1: Arc<dyn Pdf>
}

impl MixturePdf {
    pub fn new(pdf0: Arc<dyn Pdf>, pdf1: Arc<dyn Pdf>) -> Self {
        Self {
            pdf0: pdf0.clone(),
            pdf1: pdf1.clone()
        }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, rng: &mut SmallRng, direction: &Vec3) -> f64 {
        // average of pdfs is also a pdf
        0.5 * self.pdf0.value(rng, direction) + 0.5 * self.pdf1.value(rng, direction)
    }

    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        if rng.gen::<f64>() < 0.5 {
            self.pdf0.generate(rng)
        } else {
            self.pdf1.generate(rng)
        }
    }
}