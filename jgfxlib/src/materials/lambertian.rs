use crate::hittables::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::colour::Colour;
use crate::vec3::Vec3;


#[derive(Default)]
pub enum DiffuseMethod {
    CosCubed,
    CosSphere,
    #[default]
    CosHemisphere
}


/// Lambertian (diffuse) material
pub struct Lambertian {
    albedo: Colour,
    diffuse_method: DiffuseMethod
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self {
            albedo,
            diffuse_method: DiffuseMethod::default()
        }
    }

    /// Set the method this image will implement diffuse
    pub fn set_diffuse_method(&mut self, method: DiffuseMethod) {
        self.diffuse_method = method;
    }

    // fn diffuse_direction(&self, rec: &HitRecord) -> Ray {
    //     let target = match self.diffuse_method {
    //         DiffuseMethod::CosCubed => &rec.p + &rec.n + Vec3::random_in_unit_sphere(),
    //         DiffuseMethod::CosSphere => &rec.p + &rec.n + Vec3::random_unit_vector(),
    //         DiffuseMethod::CosHemisphere => &rec.p + Vec3::random_in_hemisphere(&rec.n),
    //     };

    //     // 0=p, 1=n
    //     Ray::new(&rec.p, &(&target - &rec.p))
    // }
}

impl Material for Lambertian {
    // Returns (attenuation, scattered_ray) as an option
    fn scatter(&self, _: Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = rec.n + Vec3::random_unit_vector();

        // catch near 0 direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.n;
        }

        let scattered = Ray::new(&rec.p, &scatter_direction);
        
        Some((self.albedo, scattered))
    }
}


