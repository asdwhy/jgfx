use std::sync::Arc;

use jrpt::{
    objects::{
        object_list,
        sphere, 
        affine, 
        Object
    }, 
    colour::Colour, 
    textures::checker_texture::CheckerTexture, 
    materials::{
        lambertian::Lambertian
    }
};

pub fn build_scene() -> Object {
    let mut world = object_list::new();

    let checker = Arc::new(CheckerTexture::new(Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)));
    let mat = Arc::new(Lambertian::from_texture(checker));

    let sphere = sphere::canonical(mat.clone());
    let mut transform = affine::new(sphere);

    affine::scale_uniform(&mut transform, 10.0);
    affine::translate(&mut transform, 0.0, -10.0, 0.0);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, transform);

    let sphere = sphere::canonical(mat.clone());
    let mut transform = affine::new(sphere);
    affine::scale_uniform(&mut transform, 10.0);
    affine::translate(&mut transform, 0.0, 10.0, 0.0);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, transform);

    world


    // let mut world = ObjectList::new();

    // let checker = Arc::new(CheckerTexture::new(Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)));
    // let mat = Arc::new(Lambertian::from_texture(checker));

    // world.add(Arc::new(MovingSphere::new(Point3::new(0.0, -10.0, 0.0), Point3::new(0.0, -10.0, 0.0), 0.0..1.0, 10.0, mat.clone())));
    // world.add(Arc::new(MovingSphere::new(Point3::new(0.0, 10.0, 0.0), Point3::new(0.0, -10.0, 0.0), 0.0..1.0, 10.0, mat.clone())));

    // world
}
