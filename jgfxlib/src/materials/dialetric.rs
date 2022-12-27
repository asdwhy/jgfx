use rand::Rng;
use rand::rngs::SmallRng;

use crate::hittables::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::colour::Colour;
use crate::utils::fmin;

pub struct Dialetric {
    ir: f64 // index of refraction
}

impl Dialetric {
    /// Creates a dialetric (refractive/transmissive) material
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction
        }
    }

    /// Schlick appromixation for reflectance
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dialetric {
    // Returns (attenuation, scattered_ray) as an option
    fn scatter(&self, rng: &mut SmallRng, ray_in: Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let attenuation = Colour::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = ray_in.dir.normalized();
        let cos_theta = fmin(-unit_direction.dot(&rec.n), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let is_total_internal_reflection = refraction_ratio * sin_theta > 1.0;

        let direction = if is_total_internal_reflection || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
            unit_direction.reflect(&rec.n)
        } else {
            unit_direction.refract(&rec.n, refraction_ratio)
        };

        let scattered = Ray::new(rec.p.clone(), direction, ray_in.time);

        Some((attenuation, scattered))
    }
}
