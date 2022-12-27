use std::{sync::Arc, ops::Range, f64::{NEG_INFINITY, INFINITY}};

use rand::rngs::SmallRng;

use crate::{hittables::Hittable, aabb::AABB, ray::Ray, point3::Point3, vec3::Vec3, utils::{fmin, fmax}};

use super::HitRecord;


pub struct RotateY {
    obj: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<AABB>
}

impl RotateY {
    /// Rotate obj by angle degrees about the Y axis
    /// obj must have bounding box
    pub fn new(obj: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        match obj.bounding_box(0.0..1.0) {
            Some(bbox) => {
                let mut min = Point3::from_value(INFINITY);
                let mut max = Point3::from_value(NEG_INFINITY);

                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let if64 = i as f64;
                            let jf64 = j as f64;
                            let kf64 = k as f64;

                            let x = if64 * bbox.maximum.x + ((1-i) as f64) * bbox.minimum.x;
                            let y = jf64 * bbox.maximum.y + ((1-j) as f64) * bbox.minimum.y;
                            let z = kf64 * bbox.maximum.z + ((1-k) as f64) * bbox.minimum.z;

                            let newx = cos_theta*x + sin_theta * z;
                            let newz = -sin_theta*x + cos_theta*z;

                            let tester = Vec3::new(newx, y, newz);

                            for c in 0..3 {
                                min[c] = fmin(min[c], tester[c]);
                                max[c] = fmax(max[c], tester[c]);
                            }
                        }
                    }
                }

                Self {
                    obj: obj.clone(),
                    sin_theta, cos_theta,
                    bounding_box: Some(AABB::new(min, max))
                }
            },
            None => Self {
                obj: obj.clone(),
                sin_theta, cos_theta,
                bounding_box: None
            }
        }

    }
}

impl Hittable for RotateY {
    fn intersect(&self, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin.clone();
        let mut dir = r.dir.clone();

        origin[0] = self.cos_theta*r.origin[0] - self.sin_theta*r.origin[2];
        origin[2] = self.sin_theta*r.origin[0] + self.cos_theta*r.origin[2];

        dir[0] = self.cos_theta*r.dir[0] - self.sin_theta*r.dir[2];
        dir[2] = self.sin_theta*r.dir[0] + self.cos_theta*r.dir[2];

        let rotated_r = Ray::new(origin, dir, r.time);

        match self.obj.intersect(rng, &rotated_r, t_min, t_max) {
            Some(mut rec) => {
                let mut p = rec.p;
                let mut n = rec.n;

                p[0] =  self.cos_theta*rec.p[0] + self.sin_theta*rec.p[2];
                p[2] = -self.sin_theta*rec.p[0] + self.cos_theta*rec.p[2];

                n[0] =  self.cos_theta*rec.n[0] + self.sin_theta*rec.n[2];
                n[2] = -self.sin_theta*rec.n[0] + self.cos_theta*rec.n[2];

                rec.p = p;
                rec.n = n;
                rec.set_face_normal(&rotated_r);
                
                Some(rec)
            },
            None => None,
        }
    }

    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        self.bounding_box.clone()
    }
}