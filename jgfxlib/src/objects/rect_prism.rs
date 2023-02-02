use std::{ops::Range, sync::Arc};
use rand::rngs::SmallRng;
use crate::{
    aabb::AABB,
    ray::Ray,
    point3::Point3,
    materials::Material,
    objects::{
        object_list, 
        Object,
        Intersection, AuxObjectData, 
        aa_rectangles::{xy_rect, xz_rect, yz_rect}
    }
};

// Rust has "box" keyword reserved so... rectangular prism!
pub struct RectangularPrism {
    // corners of the prism
    min: Point3,
    max: Point3,
    sides: Arc<Object> // object list
}

/// Create rectangular prism defined by the corners p0, p1
pub fn new(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> Object {
    let mut sides = object_list::new();

    object_list::add(&mut sides, xy_rect::new(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone()));
    object_list::add(&mut sides, xy_rect::new(p0.x, p1.x, p0.y, p1.y, p0.z, material.clone()));

    object_list::add(&mut sides, xz_rect::new(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone()));
    object_list::add(&mut sides, xz_rect::new(p0.x, p1.x, p0.z, p1.z, p0.y, material.clone()));

    object_list::add(&mut sides, yz_rect::new(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone()));
    object_list::add(&mut sides, yz_rect::new(p0.y, p1.y, p0.z, p1.z, p0.x, material.clone()));

    let data = RectangularPrism {
        min: p0,
        max: p1,
        sides: Arc::new(sides)
    };

    Object {
        intersect, bounding_box,
        aux: AuxObjectData::RectangularPrism(data)
    }
}

/// Create canonical rectangular prism defined by the corners P0(0.0,0.0,0.0), P1(1.0,1.0,1.0)
pub fn canonical(material: Arc<dyn Material>) -> Object {
    let p0 = Point3::zero();
    let p1 = Point3::from_value(1.0);

    new(p0, p1, material)
}


fn bounding_box(obj: &Object, _: Range<f64>) -> Option<AABB> {
    let aux = if let AuxObjectData::RectangularPrism(aux) = &obj.aux { aux } else { panic!("Could not extract RectangularPrism from aux data") };

    Some(AABB::new(aux.min, aux.max))
}

fn intersect(obj: &Object, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let aux = if let AuxObjectData::RectangularPrism(aux) = &obj.aux { aux } else { panic!("Could not extract RectangularPrism from aux data") };

    (aux.sides.intersect)(&aux.sides, rng, r, t_min, t_max)
}
