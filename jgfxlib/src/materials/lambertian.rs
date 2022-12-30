use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::hittables::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::colour::Colour;
use crate::textures::Texture;
use crate::textures::solid_colour::SolidColour;
use crate::vec3::Vec3;

pub struct Lambertian {
    albedo: Arc<dyn Texture>
}

impl Lambertian {
    /// Creates a Lambertian (diffused) material from a colour
    pub fn new(albedo: Colour) -> Self {
        Self {
            albedo: Arc::new(SolidColour::from_rgb(albedo.x, albedo.y, albedo.z)),
        }
    }

    pub fn from_texture(albedo: Arc<dyn Texture>) -> Self {
        Self {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Lambertian {
    // Returns (attenuation, scattered_ray) as an option
    fn scatter(&self, rng: &mut SmallRng, ray_in: Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = &rec.n + Vec3::random_unit_vector(rng);

        // catch near 0 direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.n.clone();
        }

        let scattered = Ray::new(rec.p.clone(), scatter_direction, ray_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Some((attenuation, scattered))
    }
}


