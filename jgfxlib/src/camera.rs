use rand::rngs::SmallRng;

use crate::{point3::Point3, vec3::Vec3, ray::Ray};

#[allow(dead_code)]
pub struct Camera {
    aspect_ratio: f64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, up: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_distance: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&lookfrom - &lookat).normalized();
        let u = up.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_distance * viewport_width * &u;
        let vertical = focus_distance * viewport_height * &v;
        let lower_left_corner = &origin - &horizontal/2.0 - &vertical/2.0 - focus_distance * &w;

        let lens_radius = aperture / 2.0;

        Self {
            aspect_ratio,
            origin,
            horizontal,
            vertical,
            u, v, w,
            lower_left_corner,
            lens_radius
        }
    }

    pub fn get_ray(&self, rng: &mut SmallRng, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = &self.u * rd.x + &self.v * rd.y;

        Ray::new(
            &self.origin + &offset, 
            &self.lower_left_corner + s * &self.horizontal + t * &self.vertical - &self.origin - &offset
        )
    }
}