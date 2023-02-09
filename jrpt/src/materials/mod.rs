use rand::rngs::SmallRng;

use crate::{
    ray::Ray, 
    objects::Intersection, 
    colour::Colour, 
    point3::Point3
};

#[allow(unused)]
pub trait Material: Send + Sync {
    /// Returns (attenuation, scattered_ray) as an option
    fn scatter(&self, rng: &mut SmallRng, ray_in: Ray, rec: &Intersection) -> Option<(Colour, Ray)>;

    /// Returns emitted colour from this material.
    /// defaults to black for all materials
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Colour {
        Colour::zero()
    }
}


pub mod lambertian;
pub mod metal;
pub mod dialetric;
pub mod diffuse_light;
pub mod isotropic;