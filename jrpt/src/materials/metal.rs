use rand::rngs::SmallRng;
use crate::{
    objects::Intersection,
    materials::Material,
    random::random_in_unit_sphere,
    colour::Colour,
    utils::fmin,
    ray::Ray
};

pub struct Metal {
    albedo: Colour,
    fuzzy: f64
}

impl Metal {
    /// Creates a metal (reflective) material
    pub fn new(albedo: Colour, fuzzy: f64) -> Self {
        Self {
            albedo,
            fuzzy: fmin(fuzzy, 1.0)
        }
    }
}

impl Material for Metal {
    // Returns (attenuation, scattered_ray) as an option
    fn scatter(&self, rng: &mut SmallRng, ray_in: Ray, rec: &Intersection) -> Option<(Colour, Ray)> {
        let reflected_dir = ray_in.dir.normalized().reflect(&rec.n);

        let scattered = Ray::new(rec.p, &reflected_dir + self.fuzzy*random_in_unit_sphere(rng), ray_in.time);
        let attenuation = self.albedo;

        if scattered.dir.dot(&rec.n) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
