use crate::{
    colour::Colour,
    point3::Point3,
    textures::Texture
};

pub struct SolidColour {
    colour_value: Colour
}

impl SolidColour {
    pub fn new(col: Colour) -> Self {
        Self {
            colour_value: col
        }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(Colour::new(r, g, b))
    }
}

impl Texture for SolidColour {
    fn value(&self, _: f64, _: f64, _: &Point3) -> Colour {
        self.colour_value.clone()
    }
}