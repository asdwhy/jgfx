/// Utilities involving randomness
/*
 NOTE: Random functions where you dont pass an RNG is not meant to be used
 during the actual rendering. These are utility random functions to be used for
 actions that are usually done once, like creating objects / materials / textures.
*/ 
use std::ops::Range;
use rand::{rngs::SmallRng, Rng, SeedableRng, thread_rng};
use crate::vec3::Vec3;

// Returns random Vec3 where all dimensions are random from [0,1)
pub fn random(rng: &mut SmallRng) -> Vec3 {
    Vec3::new(rng.gen(), rng.gen(), rng.gen())
}

/// Returns random Vec3 where all dimension are between min and max
pub fn random_in_range(rng: &mut SmallRng, min: f64, max: f64) -> Vec3 {
    Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
}

/// Returns random vector in the unit sphere
pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Vec3 {
    loop {
        let p = random_in_range(rng, -1.0, 1.0);
        if p.length_squared() >= 1.0 { continue; }

        return p;
    }
}

/// Returns random unit vector
pub fn random_unit_vector(rng: &mut SmallRng) -> Vec3 {
    random_in_unit_sphere(rng).normalized()
}

/// Returns random vector in hemisphere aligned with normal
pub fn random_in_hemisphere(rng: &mut SmallRng, normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere(rng);
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

/// Returns random vector in unit disk on x-y plane
pub fn random_in_unit_disk(rng: &mut SmallRng) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }

        return p;
    }
}

pub fn random_i32(range: Range<i32>) -> i32 {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    rng.gen_range(range)
}
