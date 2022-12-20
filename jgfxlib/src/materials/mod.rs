use crate::{ray::Ray, hittables::HitRecord, colour::Colour};


pub trait Material: Send + Sync {
    /// Returns (attenuation, scattered_ray) as an option
    fn scatter(&self, ray_in: Ray, rec: &HitRecord) -> Option<(Colour, Ray)>;
}


pub mod lambertian;
pub mod metal;
pub mod dialetric;