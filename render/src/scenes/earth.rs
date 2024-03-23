use std::sync::Arc;

use jrpt::{
    materials::lambertian::Lambertian,
    objects::{
        affine::{self},
        object_list::{self},
        sphere::{self},
        Object,
    },
    textures::image_texture::ImageTexture,
};

pub fn build_scene() -> Object {
    let mut world = object_list::new();

    let earth_texture = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));

    let sphere = sphere::canonical(earth_surface);
    let mut transform = affine::new(sphere);
    affine::rotate_z(&mut transform, 0.4);
    affine::scale_uniform(&mut transform, 2.0);
    affine::translate(&mut transform, 1.0, 1.0, 0.0);
    affine::set_inverse(&mut transform);

    object_list::add(&mut world, transform);

    world
}
