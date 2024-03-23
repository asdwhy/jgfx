use core::panic;
use std::{sync::Arc};
use wavefront::Obj;
use crate::{
    objects::{triangle, object_list, Object, AuxObjectData},
    materials::{Material},
    vec3::Vec3, point3::Point3
};

/// Create a triangle mesh from .obj file at given filename
pub fn new_mesh(filename: String, material: Arc<dyn Material>) -> Object {
    let model = match Obj::from_file(filename.clone()) {
        Err(err) => {
            eprintln!("Error parsing {filename}: {err}");
            panic!();
        },
        Ok(model) => model
    };

    match create_mesh(model, material) {
        Some(obj) => obj,
        None => {
            eprintln!("Error parsing {filename}");
            panic!();
        },
    }
}

fn create_mesh(model: Obj, material: Arc<dyn Material>) -> Option<Object> {
    let mut list = object_list::new();
    
    for triangle in model.triangles() {
        // normal of triangle
        let v0 = triangle[0];
        let n = v0.normal()?;
        let n = Vec3::new(n[0] as f64, n[1] as f64, n[2] as f64);

        // corner 1
        let p0 = v0.position();
        let p0 = Point3::new(p0[0] as f64, p0[1] as f64, p0[2] as f64);
        let mut uv = v0.uv()?;

        // corner 2
        let v1 = triangle[1];
        let p1 = v1.position();
        let p1 = Point3::new(p1[0] as f64, p1[1] as f64, p1[2] as f64);
        uv[0] += v1.uv()?[0];
        uv[1] += v1.uv()?[1];

        // corner 3
        let v2 = triangle[2];
        let p2 = v2.position();
        let p2 = Point3::new(p2[0] as f64, p2[1] as f64, p2[2] as f64);
        uv[0] += v2.uv()?[0];
        uv[1] += v2.uv()?[1];
        
        // average uv
        uv[0] /= 3.0;
        uv[1] /= 3.0;

        // TODO: interpolate uv value instead of simple average
        let t = triangle::new(p0, p1, p2, Some(n), Some((uv[0] as f64, uv[1] as f64)), material.clone());
        object_list::add(&mut list, t);
    }

    let aux = if let AuxObjectData::ObjectList(aux) = &list.aux { aux } else { panic!("Could not extract ObjectList from aux data") };
    println!("Created mesh with {} triangles", aux.objects.len());

    Some(list)
}