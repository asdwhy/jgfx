use crate::{
    point3::Point3,
    ray::Ray,
    utils::{fmin, fmax}
};

/// Axis Aligned Bounding Box
#[derive(Clone)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3
}


impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            minimum: a,
            maximum: b
        }
    }

    // Intersect with axis aligned bounding box
    pub fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        // for loop over each dimension is unwrapped for performance
        let mut t_min = t_min;
        let mut t_max = t_max;

        // x dim
        let mut inv_d = 1.0 / r.dir.x;
        let mut t0 = (self.minimum.x - r.origin.x) * inv_d;
        let mut t1 = (self.maximum.x - r.origin.x) * inv_d;

        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }

        t_min = fmax(t0, t_min);
        t_max = fmin(t1, t_max);

        if t_max <= t_min {
            return false;
        }

        // y dim
        inv_d = 1.0 / r.dir.y;
        t0 = (self.minimum.y - r.origin.y) * inv_d;
        t1 = (self.maximum.y - r.origin.y) * inv_d;

        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }

        t_min = fmax(t0, t_min);
        t_max = fmin(t1, t_max);

        if t_max <= t_min {
            return false;
        }

        // z dim
        inv_d = 1.0 / r.dir.z;
        t0 = (self.minimum.z - r.origin.z) * inv_d;
        t1 = (self.maximum.z - r.origin.z) * inv_d;

        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }

        t_min = fmax(t0, t_min);
        t_max = fmin(t1, t_max);

        if t_max <= t_min {
            return false;
        }

        true
    }
}


/// Returns a box surrounding both box0 and box1
pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Point3::new(
        fmin(box0.minimum.x, box1.minimum.x),
        fmin(box0.minimum.y, box1.minimum.y),
        fmin(box0.minimum.z, box1.minimum.z)
    );

    let big = Point3::new(
        fmax(box0.maximum.x, box1.maximum.x),
        fmax(box0.maximum.y, box1.maximum.y),
        fmax(box0.maximum.z, box1.maximum.z)
    );

    AABB::new(small, big)
}