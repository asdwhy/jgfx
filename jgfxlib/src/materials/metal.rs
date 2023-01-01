use rand::rngs::SmallRng;

use crate::hittables::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::colour::Colour;
use crate::utils::{fmin};
use crate::vec3::Vec3;

use super::ScatterRecord;

pub struct Metal {
    albedo: Colour,
    fuzzy: f64
}

impl Metal {
    /// Creates a metal (reflective) material
    pub fn new(albedo: Colour, fuzzy: f64) -> Self {
        Self {
            albedo,
            fuzzy: fmin(fuzzy, 1.0)
        }
    }
}

impl Material for Metal {
    // Returns (attenuation, scattered_ray) as an option
    fn scatter(&self, rng: &mut SmallRng, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray_in.dir.normalized().reflect(&rec.n);

        let specular_ray = Ray::new(rec.p, reflected + self.fuzzy*Vec3::random_in_unit_sphere(rng), ray_in.time);
        
        if specular_ray.dir.dot(&rec.n) <= 0.0 {
            return None;
        }
        
        let attenuation = self.albedo.clone();

        let srec = ScatterRecord::new(Some(specular_ray), attenuation, None);

        Some(srec)        
    }
}
