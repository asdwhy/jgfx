use image::Rgb;

use crate::{vec3::Vec3, utils::clamp};

pub type Colour = Vec3;

impl Colour {
    pub fn to_rgb(&self) -> Rgb<u8> {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        r = r.sqrt();
        g = g.sqrt();
        b = b.sqrt();

        Rgb([
            (256.0 * clamp(r, 0.0, 0.999)) as u8, 
            (256.0 * clamp(g, 0.0, 0.999)) as u8, 
            (256.0 * clamp(b, 0.0, 0.999)) as u8, 
        ])
    }
}