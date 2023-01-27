use std::{sync::Arc};

use jgfxlib::{
    objects::{
        object_list::ObjectList, 
        triangle::Triangle
    }, 
    textures::{checker_texture::CheckerTexture}, 
    materials::{
        lambertian::Lambertian
    }, colour::Colour, point3::Point3
};

pub fn build_scene() -> ObjectList {
    let mut world = ObjectList::new();
    let checker = Arc::new(CheckerTexture::new(Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)));
    let ground_material = Arc::new(Lambertian::from_texture(checker));

    let tr = Triangle::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(3.0, 3.0, 3.0),
        None,
        Some((0.5, 0.5)),
        ground_material.clone()
    );


    world.add(Arc::new(tr));

    world
}
