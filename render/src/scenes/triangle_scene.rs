use std::sync::Arc;

use jrpt::{
    colour::Colour,
    materials::lambertian::Lambertian,
    objects::{
        object_list::{self},
        triangle::{self},
        Object,
    },
    point3::Point3,
    textures::checker_texture::CheckerTexture,
};

pub fn build_scene() -> Object {
    let mut world = object_list::new();
    let checker = Arc::new(CheckerTexture::new(
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::from_texture(checker));

    let tr = triangle::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(3.0, 3.0, 3.0),
        None,
        Some((0.5, 0.5)),
        ground_material.clone(),
    );

    object_list::add(&mut world, tr);

    world
}
