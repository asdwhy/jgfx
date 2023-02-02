use std::{ops::Range, sync::Arc};
use nalgebra::{Vector4, Matrix4};
use rand::rngs::SmallRng;
use crate::{
    vec3::Vec3, 
    ray::Ray, 
    objects::{Intersection, Object, AuxObjectData}, 
    point3::Point3, aabb::AABB, utils::{fmin, fmax}
};

/// Affine transformations
pub struct Affine {
    object: Arc<Object>,             // object being wrapped with transformation
    transformed: bool,          // flag to denote non identity transform
    mat_t: Matrix4<f64>,        // note these matrices are stored as column vectors!
    mat_t_inv: Matrix4<f64>
}

/// Create affinely transformable object given by passed Object
pub fn new(object: Object) -> Object {
    let data = Affine {
        object: Arc::new(object),
        transformed: false,
        mat_t: Matrix4::identity(),
        mat_t_inv: Matrix4::identity(),
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::Affine(data)
    }
}

fn inverse_ray_transform(transformation: &Affine, r: &Ray) -> Ray {
    let origin = Vector4::new(r.origin.x, r.origin.y, r.origin.z, 1.0);
    let o = transformation.mat_t_inv * origin;
    
    let dir = Vector4::new(r.dir.x, r.dir.y, r.dir.z, 1.0);
    let mut d = transformation.mat_t_inv * dir;
    
    let inv = transformation.mat_t_inv.data.0;
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

fn hitrec_transform(transformation: &Affine, rec: &mut Intersection, r: &Ray) {
    rec.p = r.at(rec.t);
    rec.n = normal_transform(transformation, &rec.n);
}

fn normal_transform(transformation: &Affine, n: &Vec3) -> Vec3 {
    let mat = transformation.mat_t_inv.data.0;
    
    Vec3::new(
        mat[0][0]*n.x + mat[0][1]*n.y + mat[0][2]*n.z, 
        mat[1][0]*n.x + mat[1][1]*n.y + mat[1][2]*n.z, 
        mat[2][0]*n.x + mat[2][1]*n.y + mat[2][2]*n.z
    ).normalized()
}

fn point_transform(transformation: &Affine, p: &Point3) -> Point3 {
    let p_ = Vector4::new(p.x, p.y, p.z, 1.0);
    let o = transformation.mat_t * p_;
    Point3::new(o.x, o.y, o.z)
}

/// Sets the inverse transformation for this affine transformation
pub fn set_inverse(obj: &mut Object) {
    let aux = if let AuxObjectData::Affine(aux) = &mut obj.aux { aux } else { panic!("Could not extract Affine from aux data") };

    aux.mat_t_inv = aux.mat_t.try_inverse().unwrap()
}

/// Rotates this transformation by theta radians about the x axis
pub fn rotate_x(obj: &mut Object, theta: f64) {
    let aux = if let AuxObjectData::Affine(aux) = &mut obj.aux { aux } else { panic!("Could not extract Affine from aux data") };

    aux.transformed = true;

    let mt = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, theta.cos(), -theta.sin(), 0.0,
        0.0, theta.sin(), theta.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    aux.mat_t = mt * aux.mat_t;
}

/// Rotates this transformation by theta radians about the y axis
pub fn rotate_y(obj: &mut Object, theta: f64) {
    let aux = if let AuxObjectData::Affine(aux) = &mut obj.aux { aux } else { panic!("Could not extract Affine from aux data") };

    aux.transformed = true;

    let mt = Matrix4::new(
        theta.cos(), 0.0, theta.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -theta.sin(), 0.0, theta.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    aux.mat_t = mt * aux.mat_t;
}

/// Rotates this transformation by theta radians about the z axis
pub fn rotate_z(obj: &mut Object, theta: f64) {
    let aux = if let AuxObjectData::Affine(aux) = &mut obj.aux { aux } else { panic!("Could not extract Affine from aux data") };

    aux.transformed = true;

    let mt = Matrix4::new(
        theta.cos(), -theta.sin(), 0.0, 0.0,
        theta.sin(), theta.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    aux.mat_t = mt * aux.mat_t;
}

/// Scales this transformation by given values for each dimension
pub fn scale(obj: &mut Object, x_scale: f64, y_scale: f64, z_scale: f64) {
    let aux = if let AuxObjectData::Affine(aux) = &mut obj.aux { aux } else { panic!("Could not extract Affine from aux data") };

    aux.transformed = true;

    let mt = Matrix4::new(
        x_scale, 0.0, 0.0, 0.0,
        0.0, y_scale, 0.0, 0.0,
        0.0, 0.0, z_scale, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    aux.mat_t = mt * aux.mat_t;
}

/// Scales this transformation by scale in all dimensions
pub fn scale_uniform(obj: &mut Object, uscale: f64) {
    scale(obj, uscale, uscale, uscale)
}

/// Translates this transformation by given values for each dimension
pub fn translate(obj: &mut Object, x_translate: f64, y_translate: f64, z_translate: f64) {
    let aux = if let AuxObjectData::Affine(aux) = &mut obj.aux { aux } else { panic!("Could not extract Affine from aux data") };

    aux.transformed = true;

    let mt = Matrix4::new(
        1.0, 0.0, 0.0, x_translate,
        0.0, 1.0, 0.0, y_translate,
        0.0, 0.0, 1.0, z_translate,
        0.0, 0.0, 0.0, 1.0
    );

    aux.mat_t = mt * aux.mat_t
}


fn bounding_box(obj: &Object, time: Range<f64>) -> Option<AABB> {
    let aux = if let AuxObjectData::Affine(aux) = &obj.aux { aux } else { panic!("Could not extract Affine from aux data") };

    match (aux.object.bounding_box)(&aux.object, time) {
        Some(bbox) => {
            // transform minimum and maximum corners
            let p1 = point_transform(&aux, &bbox.minimum);
            let p2 = point_transform(&aux, &bbox.maximum);

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
            let p3 = point_transform(&aux, &p3);
            let p4 = point_transform(&aux, &p4);
            let p5 = point_transform(&aux, &p5);
            let p6 = point_transform(&aux, &p6);
            let p7 = point_transform(&aux, &p7);
            let p8 = point_transform(&aux, &p8);

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

fn intersect(obj: &Object, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::Affine(aux) = &obj.aux { aux } else { panic!("Could not extract Affine from aux data") };

    let rt = inverse_ray_transform(&aux, r);
    return match (aux.object.intersect)(&aux.object, rng, &rt, t_min, t_max) {
        Some(mut rec) => {
            hitrec_transform(&aux, &mut rec, r);
            Some(rec)
        },
        None => None,
    }
}

