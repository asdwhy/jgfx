use std::sync::Arc;

use jgfxlib::{
    hittables::{
        hittable_list::HittableList, sphere::Sphere
    }, 
    textures::{noise_texture::NoiseTexture}, 
    materials::{
        lambertian::Lambertian
    }, 
    point3::Point3
};

pub fn build_scene() -> HittableList {
    let mut world = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let mat = Arc::new(Lambertian::from_texture(perlin_texture));

    world.add(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat.clone())));

    world
}
