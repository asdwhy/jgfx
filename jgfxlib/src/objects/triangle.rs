// Rectangle aligned with X-Y Plane

use std::{sync::Arc, ops::Range};
use rand::rngs::SmallRng;
use crate::{
    materials::Material, 
    aabb::AABB, 
    point3::Point3, 
    ray::Ray, vec3::Vec3,
    objects::{Object, Intersection}, utils::{fmin, fmax}
};

pub struct Triangle {
    material: Arc<dyn Material>,
    p0: Point3,
    p1: Point3,
    p2: Point3,
    n: Vec3,
    uv: Option<(f64,f64)>
}

impl Triangle {
    /// Create triangle defined by corners P0, P1, P2, with given normal vector (optional)
    pub fn new(p0: Point3, p1: Point3, p2: Point3, n: Option<Vec3>, uv: Option<(f64,f64)>, material: Arc<dyn Material>) -> Self {
        let n = match n {
            Some(n) => n,
            None => (&p1-&p0).cross(&(&p2-&p1)),
        };
        
        Self {
            p0, p1, p2, n, uv,
            material: material.clone()
        }
    }

    /// Create canonical triangle on X-Y plane defined by corners P0(0.0, 0.0, 0.0), P1(0.0, 1.0, 0.0), P2(1.0, 0.0, 0.0) 
    pub fn canonical(material: Arc<dyn Material>) -> Self {
        let p0 = Point3::zero();
        let p1 = Point3::new(0.0,1.0,0.0);
        let p2 = Point3::new(1.0,0.0,0.0);
        let n = (&p1-&p0).cross(&(&p2-&p1));
        
        Self {
            p0, p1, p2, n,
            uv: None,
            material: material.clone()
        }
    }
}

impl Object for Triangle {
    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount
        let x_min = fmin(fmin(self.p0.x, self.p1.x), self.p2.x);
        let y_min = fmin(fmin(self.p0.y, self.p1.y), self.p2.y);
        let z_min = fmin(fmin(self.p0.z, self.p1.z), self.p2.z);

        let x_max = fmax(fmax(self.p0.x, self.p1.x), self.p2.x);
        let y_max = fmax(fmax(self.p0.y, self.p1.y), self.p2.y);
        let z_max = fmax(fmax(self.p0.z, self.p1.z), self.p2.z);

        let tolerance = Vec3::from_value(0.0001);

        Some(AABB::new(
            Point3::new(x_min, y_min, z_min) - &tolerance, 
            Point3::new(x_max, y_max, z_max) + &tolerance
        ))
    }

    fn intersect(&self, _: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let t = (&self.p0 - &r.origin).dot(&self.n)/(r.dir.dot(&self.n));

        if t < t_min || t > t_max {
            return None;
        }

        let p = r.at(t);
        let n = self.n;

        let uv = match self.uv {
            Some(uv) => uv,
            None => {
                // compute uv
                todo!()
            }
        };

        let v = &p - &self.p0;

        // check side 1
        let e01 = &self.p1 - &self.p0;
        if e01.cross(&v).dot(&n) < 0.0 {
            return None;
        }

        // check side 2
        let v = &p - &self.p1;
        let e12 = &self.p2 - &self.p1;
        if e12.cross(&v).dot(&n) < 0.0 {
            return None;
        }

        // check side 3
        let v = &p - &self.p2;
        let e20 = &self.p0 - &self.p2;
        if e20.cross(&v).dot(&n) < 0.0 {
            return None;
        }

        let mut rec = Intersection::new(t, p, n, &self.material, uv.0, uv.1);
        rec.set_face_normal(r);
        
        Some(rec)
    }
}