use std::sync::Arc;
use rand::rngs::SmallRng;
use crate::{
    textures::{Texture, 
       solid_colour::SolidColour}, 
       colour::Colour, 
       ray::Ray, 
       objects::Intersection, 
       random::random_in_unit_sphere,
       materials::Material
};

pub struct Isotropic {
    albedo: Arc<dyn Texture>
}

impl Isotropic {
    /// Creates isotropic material (material that looks same from every angle)
    pub fn new(colour: Colour) -> Self {
        Self {
            albedo: Arc::new(SolidColour::new(colour))
        }
    }

    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self {
            albedo: texture.clone()
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, rng: &mut SmallRng, ray_in: Ray, rec: &Intersection) -> Option<(Colour, Ray)> {

        let scattered = Ray::new(rec.p, random_in_unit_sphere(rng), ray_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Some((attenuation, scattered))
    }
}