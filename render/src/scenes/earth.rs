use std::sync::Arc;

use jgfxlib::{
    hittables::{
        hittable_list::HittableList, sphere::Sphere
    }, 
    textures::{image_texture::ImageTexture}, 
    materials::{
        lambertian::Lambertian
    }, 
    point3::Point3
};

pub fn build_scene() -> HittableList {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));

    world.add(Arc::new(Sphere::new(Point3::zero(), 2.0, earth_surface)));

    world
}
