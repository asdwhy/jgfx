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

    pub fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        // check if intersect in each dimension
        for i in 0..3 {
            let inv_d = 1.0 / r.dir[i];
            let mut t0 = (self.minimum[i] - r.origin[i]) * inv_d;
            let mut t1 = (self.maximum[i] - r.origin[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = fmax(t0, t_min);
            t_max = fmin(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }

        return true;
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