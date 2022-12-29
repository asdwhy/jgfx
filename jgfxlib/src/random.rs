use std::{ops::Range, f64::consts::PI};

/// Utilities involving randomness
use rand::{rngs::SmallRng, Rng, SeedableRng, thread_rng};

use crate::vec3::Vec3;

impl Vec3 {
    // Returns Vec3 where all dimensions are random from [0,1)
    pub fn random(rng: &mut SmallRng) -> Vec3 {
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_in_range(rng: &mut SmallRng, min: f64, max: f64) -> Vec3 {
        Self::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    /// Returns random vector in the unit sphere
    pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Self {
        loop {
            let p = Self::random_in_range(rng, -1.0, 1.0);
            if p.length_squared() >= 1.0 { continue; }

            return p;
        }
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
        loop {
            let p = Self::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn random_cosine_direction(rng: &mut SmallRng) -> Vec3 {
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();
        let z = (1.0-r2).sqrt();

        let phi = 2.0*PI*r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();

        Vec3::new(x, y, z)
    }

    pub fn random_to_sphere(rng: &mut SmallRng, radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();

        let z = 1.0 + r2*((1.0-(radius*radius/distance_squared)).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos()*((1.0-z*z).sqrt());
        let y = phi.sin()*((1.0-z*z).sqrt());

        Vec3::new(x, y, z)
    }
}

/*
 NOTE: Random functions where you dont pass an RNG is not meant to be used
 during the actual rendering. These are utility random functions to be used for
 actions that are usually done once, like creating objects / materials / textures.
*/ 

pub fn random_i32(range: Range<i32>) -> i32 {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    rng.gen_range(range)
}
