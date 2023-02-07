use std::ops::Range;
use rand::{rngs::SmallRng, Rng};
use crate::{
    point3::Point3, 
    vec3::Vec3, 
    ray::Ray, 
    random::random_in_unit_disk
};

#[allow(unused)]
pub struct Camera {
    aspect_ratio: f64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,        // for generating rays
    u: Vec3, v: Vec3, w: Vec3,      // camera coordinate frame
    lens_radius: f64,               // for depth of field
    time: Range<f64>,               // shutter open close times for motion blur
}

impl Camera {
    pub fn new(
        lookfrom: Vec3, 
        lookat: Vec3, 
        up: Vec3, 
        vfov: f64, 
        aspect_ratio: f64, 
        aperture: f64, 
        focus_distance: f64,
        time: Range<f64>
    ) -> Self {
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
            lens_radius,
            time
        }
    }

    // get new ray to trace from this camera
    pub fn get_ray(&self, rng: &mut SmallRng, s: f64, t: f64) -> Ray {
        let origin: Point3;
        let dir: Vec3;
        
        if self.lens_radius == 0.0 { // pinhole camera => everything is in focus
            origin = self.origin;
            dir = &self.lower_left_corner + s * &self.horizontal + t * &self.vertical - &self.origin;
        } else {
            let rd = self.lens_radius * random_in_unit_disk(rng);
            let offset = &self.u * rd.x + &self.v * rd.y;

            origin = &self.origin + &offset;
            dir = &self.lower_left_corner + s * &self.horizontal + t * &self.vertical - &self.origin - &offset;
        }
        
        let ray_time = if self.time.is_empty() { 0.0 } else { rng.gen_range(self.time.clone()) };

        Ray::new(origin, dir, ray_time)
    }
}