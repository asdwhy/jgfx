use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{textures::{Texture, solid_colour::SolidColour}, colour::Colour, ray::Ray, hittables::HitRecord, point3::Point3};

use super::Material;


pub struct DiffuseLight {
    emit: Arc<dyn Texture>
}

impl DiffuseLight {
    /// Creates a diffused light from a colour
    pub fn new(colour: Colour) -> Self {
        Self {
            emit: Arc::new(SolidColour::new(colour))
        }
    }

    pub fn from_texture(emit: Arc<dyn Texture>) -> Self {
        Self {
            emit: emit.clone()
        }
    }
}

impl Material for DiffuseLight {
    // This light doesn't scatter light
    fn scatter(&self, _: &mut SmallRng, _: Ray, _: &HitRecord) -> Option<(Colour, Ray)> {
        None
    }

    // This light emits light based on its texture
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Colour {
        self.emit.value(u, v, p)
    }
}