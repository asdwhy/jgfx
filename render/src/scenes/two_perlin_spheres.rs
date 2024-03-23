use std::sync::Arc;

use jrpt::{
    materials::lambertian::Lambertian,
    objects::{
        object_list::{self},
        sphere::{self},
        Object,
    },
    point3::Point3,
    textures::noise_texture::NoiseTexture,
};

pub fn build_scene() -> Object {
    let mut world = object_list::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let mat = Arc::new(Lambertian::from_texture(perlin_texture));

    let sphere1 = sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat.clone());
    let sphere2 = sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat.clone());

    object_list::add(&mut world, sphere1);
    object_list::add(&mut world, sphere2);

    world
}
