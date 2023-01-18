use std::{sync::Arc};

use jgfxlib::{
    hittables::{
        hittable_list::HittableList, sphere::Sphere,
    }, 
    textures::{image_texture::ImageTexture}, 
    materials::{
        lambertian::Lambertian
    }, 
    point3::Point3,
};

pub fn build_scene() -> HittableList {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));

    let mut sphere = Sphere::new(earth_surface);
    // sphere.transform.rotate_z(0.4);
    // sphere.transform.scale_uniform(2.0);
    // sphere.transform.translate(1.0, 1.0, 0.0);
    // sphere.transform.set_inverse();

    world.add(Arc::new(sphere));

    world.transform.rotate_z(0.4);
    world.transform.scale_uniform(2.0);
    world.transform.translate(1.0, 1.0, 0.0);
    world.transform.set_inverse();

    world
}
