use std::sync::Arc;

use crate::materials::Material;
use crate::ray::Ray;

use crate::hittables::HitRecord;

use super::Hittable;

pub trait Object: Send + Sync {
    fn new(material: Arc<dyn Material>) -> Self where Self: Sized;
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


impl<T> Hittable for T
where
    T: Object
{
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.intersect(r, t_min, t_max)
    }
}

pub mod sphere;
