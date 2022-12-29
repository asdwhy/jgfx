use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{textures::{Texture, solid_colour::SolidColour}, colour::Colour, ray::Ray, hittables::HitRecord};

use super::{Material, ScatterRecord};


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
    fn scatter(&self, _: &mut SmallRng, _: &Ray, _: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    // This light emits light based on its texture
    fn emitted(&self, _: &Ray, rec: &HitRecord) -> Colour {
        if rec.front_face { // only emit light from the front facing side
            self.emit.value(rec.u, rec.v, &rec.p)
        } else {
            Colour::zero()
        }
    }
}