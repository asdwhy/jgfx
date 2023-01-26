use std::{sync::Arc, ops::Range};

use nalgebra::{Vector4, Matrix4};
use rand::rngs::SmallRng;
use crate::{
    vec3::Vec3, 
    ray::Ray, 
    objects::{Intersection, Object}, 
    point3::Point3, aabb::AABB, utils::{fmin, fmax}
};

/// Affine transformations
pub struct Affine {
    object: Arc<dyn Object>,
    transformed: bool,          // flag to denote non identity transform
    mat_t: Matrix4<f64>,        // note these matrices are stored as column vectors!
    mat_t_inv: Matrix4<f64>
}

impl Affine {
    /// Create affinely transformable object given by passed Object
    pub fn new(object: Arc<dyn Object>) -> Self {
        Self {
            object: object.clone(),
            transformed: false,
            mat_t: Matrix4::identity(),
            mat_t_inv: Matrix4::identity(),
        }

    }

    fn inverse_ray_transform(&self, r: &Ray) -> Ray {
        let origin = Vector4::new(r.origin.x, r.origin.y, r.origin.z, 1.0);
        let o = self.mat_t_inv * origin;
        
        let dir = Vector4::new(r.dir.x, r.dir.y, r.dir.z, 1.0);
        let mut d = self.mat_t_inv * dir;
        
        let inv = self.mat_t_inv.data.0;
        let t = Vector4::new(
            inv[3][0],
            inv[3][1],
            inv[3][2], 
            inv[3][3]
        );

        d = d - t;

        let origin = Vec3::new(o.x, o.y, o.z);
        let dir = Vec3::new(d.x, d.y, d.z);

        Ray::new(origin, dir, r.time.clone())
    }

    fn hitrec_transform(&self, rec: &mut Intersection, r: &Ray) {
        rec.p = r.at(rec.t);
        rec.n = Self::normal_transform(&self, &rec.n);
    }

    fn normal_transform(&self, n: &Vec3) -> Vec3 {
        let mat = self.mat_t_inv.data.0;
        
        Vec3::new(
            mat[0][0]*n.x + mat[0][1]*n.y + mat[0][2]*n.z, 
            mat[1][0]*n.x + mat[1][1]*n.y + mat[1][2]*n.z, 
            mat[2][0]*n.x + mat[2][1]*n.y + mat[2][2]*n.z
        ).normalized()
    }

    fn point_transform(&self, p: &Point3) -> Point3 {
        let p_ = Vector4::new(p.x, p.y, p.z, 1.0);
        let o = self.mat_t * p_;
        Point3::new(o.x, o.y, o.z)
    }

    /// Sets the inverse transformation for this affine transformation
    pub fn set_inverse(&mut self) {
        self.mat_t_inv = self.mat_t.try_inverse().unwrap()
    }

    /// Rotates this transformation by theta radians about the x axis
    pub fn rotate_x(&mut self, theta: f64) {
        self.transformed = true;

        let mt = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, theta.cos(), -theta.sin(), 0.0,
            0.0, theta.sin(), theta.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        self.mat_t = mt * self.mat_t;
    }

    /// Rotates this transformation by theta radians about the y axis
    pub fn rotate_y(&mut self, theta: f64) {
        self.transformed = true;

        let mt = Matrix4::new(
            theta.cos(), 0.0, theta.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -theta.sin(), 0.0, theta.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        self.mat_t = mt * self.mat_t;
    }

    /// Rotates this transformation by theta radians about the z axis
    pub fn rotate_z(&mut self, theta: f64) {
        self.transformed = true;

        let mt = Matrix4::new(
            theta.cos(), -theta.sin(), 0.0, 0.0,
            theta.sin(), theta.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        self.mat_t = mt * self.mat_t;
    }

    /// Scales this transformation by given values for each dimension
    pub fn scale(&mut self, x_scale: f64, y_scale: f64, z_scale: f64) {
        self.transformed = true;

        let mt = Matrix4::new(
            x_scale, 0.0, 0.0, 0.0,
            0.0, y_scale, 0.0, 0.0,
            0.0, 0.0, z_scale, 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        self.mat_t = mt * self.mat_t;
    }

    /// Scales this transformation by scale in all dimensions
    pub fn scale_uniform(&mut self, scale: f64) {
        self.transformed = true;

        Self::scale(self, scale, scale, scale);
    }

    /// Tralsates this transformation by given values for each dimension
    pub fn translate(&mut self, x_translate: f64, y_translate: f64, z_translate: f64) {
        self.transformed = true;

        let mt = Matrix4::new(
            1.0, 0.0, 0.0, x_translate,
            0.0, 1.0, 0.0, y_translate,
            0.0, 0.0, 1.0, z_translate,
            0.0, 0.0, 0.0, 1.0
        );

        self.mat_t = mt * self.mat_t
    }
}

impl Object for Affine {
    fn bounding_box(&self, time: Range<f64>) -> Option<AABB> {
        match self.object.bounding_box(time) {
            Some(bbox) => {
                // transform minimum and maximum corners
                let p1 = self.point_transform(&bbox.minimum);
                let p2 = self.point_transform(&bbox.maximum);

                // get other corners of bounding box
                let dx = bbox.maximum.x - bbox.minimum.x;
                let dy = bbox.maximum.y - bbox.minimum.y;
                let dz = bbox.maximum.z - bbox.minimum.z;

                let p3 = p1 + Point3::new(0.0, 0.0, dz);
                let p4 = p1 + Point3::new(0.0, dy, 0.0);
                let p5 = p1 + Point3::new(0.0, dy, dz);
                let p6 = p1 + Point3::new(dx, 0.0, 0.0);
                let p7 = p1 + Point3::new(dx, 0.0, dz);
                let p8 = p1 + Point3::new(dx, dy, 0.0);

                // transform other corners as well
                let p3 = self.point_transform(&p3);
                let p4 = self.point_transform(&p4);
                let p5 = self.point_transform(&p5);
                let p6 = self.point_transform(&p6);
                let p7 = self.point_transform(&p7);
                let p8 = self.point_transform(&p8);

                // take min and max values as new bounding box
                let min_x = fmin(fmin(fmin(fmin(fmin(fmin(fmin(p1.x, p2.x), p3.x), p4.x), p5.x), p6.x), p7.x), p8.x);
                let min_y = fmin(fmin(fmin(fmin(fmin(fmin(fmin(p1.y, p2.y), p3.y), p4.y), p5.y), p6.y), p7.y), p8.y);
                let min_z = fmin(fmin(fmin(fmin(fmin(fmin(fmin(p1.z, p2.z), p3.z), p4.z), p5.z), p6.z), p7.z), p8.z);

                let max_x = fmax(fmax(fmax(fmax(fmax(fmax(fmax(p1.x, p2.x), p3.x), p4.x), p5.x), p6.x), p7.x), p8.x);
                let max_y = fmax(fmax(fmax(fmax(fmax(fmax(fmax(p1.y, p2.y), p3.y), p4.y), p5.y), p6.y), p7.y), p8.y);
                let max_z = fmax(fmax(fmax(fmax(fmax(fmax(fmax(p1.z, p2.z), p3.z), p4.z), p5.z), p6.z), p7.z), p8.z);

                // transform box according to affine transformation
                Some(AABB::new(
                    Point3::new(min_x, min_y, min_z),
                    Point3::new(max_x, max_y, max_z)
                ))
            },
            None => None,
        }
    }

    fn intersect(&self, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let rt = self.inverse_ray_transform(r);
        return match self.object.intersect(rng, &rt, t_min, t_max) {
            Some(mut rec) => {
                self.hitrec_transform(&mut rec, r);
                Some(rec)
            },
            None => None,
        }
    }
}
