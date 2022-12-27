use std::sync::Arc;

use jgfxlib::{
    hittables::{
        hittable_list::HittableList, sphere::Sphere
    }, 
    colour::Colour, 
    textures::checker_texture::CheckerTexture, 
    materials::{
        lambertian::Lambertian
    }, 
    point3::Point3
};

pub fn build_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)));
    let mat = Arc::new(Lambertian::from_texture(checker));

    world.add(Arc::new(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, mat.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, mat.clone())));

    world
}
