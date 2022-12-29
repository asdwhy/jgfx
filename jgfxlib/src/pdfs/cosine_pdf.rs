use std::f64::consts::PI;

use rand::rngs::SmallRng;

use crate::{onb::Onb, vec3::Vec3};

use crate::pdfs::Pdf;


pub struct CosinePdf {
    uvw: Onb
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: Onb::new(w)
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, _: &mut SmallRng, direction: &Vec3) -> f64 {
        let cosine = direction.normalized().dot(&self.uvw.w());
        if cosine <= 0.0 { 0.0 } else { cosine/PI }
    }

    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.uvw.local_vec(&Vec3::random_cosine_direction(rng))
    }
}