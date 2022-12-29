// probability density functions

use rand::rngs::SmallRng;

use crate::vec3::Vec3;

pub trait Pdf {
    fn value(&self, rng: &mut SmallRng, direction: &Vec3) -> f64;
    fn generate(&self, rng: &mut SmallRng) -> Vec3;
}

pub mod cosine_pdf;
pub mod hittable_pdf;
pub mod mixture_pdf;