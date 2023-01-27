use std::sync::Arc;

use jgfxlib::{
    objects::{
        object_list::ObjectList,
        sphere::Sphere, 
        affine::Affine
    }, 
    colour::Colour, 
    textures::checker_texture::CheckerTexture, 
    materials::{
        lambertian::Lambertian
    }
};

pub fn build_scene() -> ObjectList {
    let mut world = ObjectList::new();

    let checker = Arc::new(CheckerTexture::new(Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)));
    let mat = Arc::new(Lambertian::from_texture(checker));

    let sphere = Sphere::canonical(mat.clone());
    let mut transform = Affine::new(Arc::new(sphere));
    transform.scale_uniform(10.0);
    transform.translate(0.0, -10.0, 0.0);
    transform.set_inverse();
    world.add(Arc::new(transform));

    let sphere = Sphere::canonical(mat.clone());
    let mut transform = Affine::new(Arc::new(sphere));
    transform.scale_uniform(10.0);
    transform.translate(0.0, 10.0, 0.0);
    transform.set_inverse();
    world.add(Arc::new(transform));

    world

    // let mut world = ObjectList::new();

    // let checker = Arc::new(CheckerTexture::new(Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)));
    // let mat = Arc::new(Lambertian::from_texture(checker));

    // world.add(Arc::new(MovingSphere::new(Point3::new(0.0, -10.0, 0.0), Point3::new(0.0, -10.0, 0.0), 0.0..1.0, 10.0, mat.clone())));
    // world.add(Arc::new(MovingSphere::new(Point3::new(0.0, 10.0, 0.0), Point3::new(0.0, -10.0, 0.0), 0.0..1.0, 10.0, mat.clone())));

    // world
}
