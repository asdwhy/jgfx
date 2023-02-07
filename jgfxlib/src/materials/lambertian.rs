use std::sync::Arc;
use rand::rngs::SmallRng;
use crate::{
    objects::Intersection,
    materials::Material,
    random::random_unit_vector,
    ray::Ray,
    colour::Colour,
    textures::{Texture, solid_colour::SolidColour}
};

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
    fn scatter(&self, rng: &mut SmallRng, ray_in: Ray, rec: &Intersection) -> Option<(Colour, Ray)> {
        let mut scatter_direction = &rec.n + random_unit_vector(rng);

        // catch near 0 direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.n;
        }

        let scattered = Ray::new(rec.p, scatter_direction, ray_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Some((attenuation, scattered))
    }
}


