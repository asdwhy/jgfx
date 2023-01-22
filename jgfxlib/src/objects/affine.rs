use std::{sync::Arc, ops::Range};

use nalgebra::{Vector4, Matrix4};
use rand::rngs::SmallRng;
use crate::{
    vec3::Vec3, 
    ray::Ray, 
    objects::{Intersection, Object}, 
    point3::Point3, aabb::AABB
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
                // transform box according to affine transformation
                Some(AABB::new(
                    self.point_transform(&bbox.minimum), 
                    self.point_transform(&bbox.maximum)
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
