use std::{sync::Arc};

use jgfxlib::{
    hittables::{
        hittable_list::HittableList, sphere::Sphere, affine::Affine,
    }, 
    textures::{image_texture::ImageTexture}, 
    materials::{
        lambertian::Lambertian
    }
};

pub fn build_scene() -> HittableList {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));

    let sphere = Sphere::new(earth_surface);
    let mut transform = Affine::new(Arc::new(sphere));
    transform.rotate_z(0.4);
    transform.scale_uniform(2.0);
    transform.translate(1.0, 1.0, 0.0);
    transform.set_inverse();

    world.add(Arc::new(transform));

    world
}
