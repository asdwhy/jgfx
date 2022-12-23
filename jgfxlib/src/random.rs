/// Implements random functions for structs

use rand::{rngs::SmallRng, Rng};
use std::f64::consts::{PI, FRAC_PI_2, TAU};

use crate::vec3::Vec3;

impl Vec3 {
    pub fn random(rng: &mut SmallRng) -> Vec3 {
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_in_range(rng: &mut SmallRng, min: f64, max: f64) -> Vec3 {
        Self::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    /// Returns random vector in the unit sphere
    pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Self {
        let theta = rng.gen_range(0.0..TAU);
        let azimuth = rng.gen_range(0.0..PI);
        Self::from_spherical(1.0, theta, azimuth)

        // loop {
        //     let p = Self::random_in_range(rng, -1.0, 1.0);
        //     if p.length_squared() >= 1.0 { continue; }

        //     return p;
        // }
    }

    /// Returns random unit vector
    pub fn random_unit_vector(rng: &mut SmallRng) -> Self {
        Self::random_in_unit_sphere(rng).normalized()
    }

    /// Returns random vector in hemisphere aligned with normal
    pub fn random_in_hemisphere(rng: &mut SmallRng, normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Returns random vector in unit disk on x-y plane
    pub fn random_in_unit_disk(rng: &mut SmallRng) -> Self {
        let theta = rng.gen_range(0.0..TAU);
        Self::from_spherical(1.0, theta, FRAC_PI_2)

        // loop {
        //     let p = Self::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        //     if p.length_squared() >= 1.0 {
        //         continue;
        //     }

        //     return p;
        // }
    }
}