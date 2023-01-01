use crate::{
    point3::Point3,
    colour::Colour
};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Colour;
}

mod perlin;
pub mod solid_colour;
pub mod checker_texture;
pub mod noise_texture;
pub mod image_texture;