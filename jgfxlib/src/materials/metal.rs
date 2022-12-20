use crate::hittables::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::colour::Colour;
use crate::utils::{fmin};
use crate::vec3::Vec3;

/// Metal (reflective) material
pub struct Metal {
    albedo: Colour,
    fuzzy: f64
}

impl Metal {
    pub fn new(albedo: Colour, fuzzy: f64) -> Self {
        Self {
            albedo,
            fuzzy: fmin(fuzzy, 1.0)
        }
    }
}

impl Material for Metal {
    // Returns (attenuation, scattered_ray) as an option
    fn scatter(&self, ray_in: Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected_dir = ray_in.dir.normalized().reflect(&rec.n);

        let scattered = Ray::new(&rec.p, &(&reflected_dir + self.fuzzy*Vec3::random_in_unit_sphere()));
        let attenuation = self.albedo;

        if scattered.dir.dot(&rec.n) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
