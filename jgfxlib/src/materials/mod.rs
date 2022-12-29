use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{ray::Ray, hittables::HitRecord, colour::Colour, pdfs::Pdf};

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Colour,
    pub pdf: Option<Arc<dyn Pdf>>
}

impl ScatterRecord {
    pub fn new(specular_ray: Option<Ray>, attenuation: Colour, pdf: Option<Arc<dyn Pdf>>) -> Self {
        Self {
            specular_ray: specular_ray, 
            attenuation, 
            pdf
        }
    }
}


#[allow(unused)]
pub trait Material: Send + Sync {
    /// Returns (attenuation, scattered_ray, pdf) as an option
    fn scatter(&self, rng: &mut SmallRng, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scaterring_pdf(&self, rng: &mut SmallRng, ray_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }

    /// Returns emitted colour from this material.
    /// defaults to black for all materials
    fn emitted(&self, ray_in: &Ray, rec: &HitRecord) -> Colour {
        Colour::zero()
    }
}


pub mod lambertian;
pub mod metal;
pub mod dialetric;
pub mod diffuse_light;
pub mod isotropic;