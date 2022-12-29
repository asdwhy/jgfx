use std::f64::consts::PI;
use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::hittables::HitRecord;
use crate::materials::Material;
use crate::pdfs::cosine_pdf::CosinePdf;
use crate::ray::Ray;
use crate::colour::Colour;
use crate::textures::Texture;
use crate::textures::solid_colour::SolidColour;

use super::ScatterRecord;

pub struct Lambertian {
    albedo: Arc<dyn Texture>
}

impl Lambertian {
    /// Creates a Lambertian (diffused) material from a colour
    pub fn new(albedo: Colour) -> Self {
        Self {
            albedo: Arc::new(SolidColour::new(albedo))
        }
    }

    pub fn from_texture(albedo: Arc<dyn Texture>) -> Self {
        Self {
            albedo: albedo.clone()
        }
    }
}

impl Material for Lambertian {
    // Returns (attenuation, scattered_ray) as an option
    fn scatter(&self, _: &mut SmallRng, _: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        let pdf = CosinePdf::new(&rec.n);

        let srec = ScatterRecord::new(None, attenuation, Some(Arc::new(pdf)));

        Some(srec)
    }

    fn scaterring_pdf(&self, _: &mut SmallRng, _: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = rec.n.dot(&scattered.dir.normalized());

        if cosine < 0.0 { 0.0 } else { cosine / PI }
    }
}


