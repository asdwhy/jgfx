use std::sync::Arc;

use jrpt::{
    colour::Colour,
    materials::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    objects::{object_list, Object, sphere, aa_rectangles::xy_rect},
    point3::Point3,
    textures::noise_texture::NoiseTexture,
};

pub fn build_scene() -> Object {
    let mut world = object_list::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let permat = Arc::new(Lambertian::from_texture(perlin_texture));

    object_list::add(
        &mut world,
        sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, permat.clone()),
    );

    object_list::add(
        &mut world,
        sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, permat.clone()),
    );

    let difflight = Arc::new(DiffuseLight::new(Colour::from_value(4.0)));

    object_list::add(
        &mut world,
        xy_rect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)
    );

    world
}
