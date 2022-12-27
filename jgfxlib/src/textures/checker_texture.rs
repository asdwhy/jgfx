use std::sync::Arc;

use crate::point3::Point3;
use crate::{textures::Texture};
use crate::colour::Colour;

use crate::textures::solid_colour::SolidColour;


pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(odd: Colour, even: Colour) -> Self {
        Self {
            odd: Arc::new(SolidColour::new(odd)),
            even: Arc::new(SolidColour::new(even))
        }
    }

    pub fn from_texture(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self {
            odd: odd.clone(),
            even: even.clone()
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Colour {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}