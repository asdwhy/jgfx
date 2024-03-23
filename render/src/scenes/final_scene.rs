use std::sync::Arc;
use rand::{rngs::SmallRng, SeedableRng, Rng};
use jrpt::{
    materials::{
        lambertian::Lambertian,
        diffuse_light::DiffuseLight,
        dialetric::Dialetric, metal::Metal
    },
    colour::Colour,
    point3::Point3,
    textures::{image_texture::ImageTexture, noise_texture::NoiseTexture},
    random::random_in_range, objects::{Object, object_list, rect_prism, bvh, aa_rectangles::xz_rect, moving_sphere, sphere, constant_medium}
};

const BOXES_PER_SIDE: i32 = 20;

pub fn build_scene() -> Object {
    let mut boxes1 = object_list::new();
    let ground_mat = Arc::new(Lambertian::new(Colour::new(0.48, 0.83, 0.53)));

    let mut rng = SmallRng::seed_from_u64(000000);

    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64)*w;
            let z0 = -1000.0 + (j as f64)*w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;
            object_list::add(&mut boxes1, rect_prism::new(Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), ground_mat.clone()));
        }
    }

    let mut world = object_list::new();
    object_list::add(&mut world, bvh::new(boxes1, 0.0..1.0));

    let light = Arc::new(DiffuseLight::new(Colour::from_value(7.0)));
    object_list::add(&mut world, xz_rect::new(123.0,423.0,147.0,412.0,554.0, light.clone()));

    let center1 = Point3::new(400.0,400.0,200.0);
    let center2 = &center1 + Point3::new(30.0,0.0,0.0);
    let moving_sphere_material = Arc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.1)));
    object_list::add(&mut world, moving_sphere::new(center1, center2, 0.0..1.0, 50.0, moving_sphere_material.clone()));
    object_list::add(&mut world, sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, Arc::new(Dialetric::new(1.5))));
    object_list::add(&mut world, sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(Colour::new(0.8, 0.8, 0.9), 1.0))));

    let boundary = sphere::new(Point3::new(360.0,150.0,145.0), 70.0, Arc::new(Dialetric::new(1.5)));
    object_list::add(&mut world, boundary);

    let boundary = sphere::new(Point3::new(360.0,150.0,145.0), 70.0, Arc::new(Dialetric::new(1.5)));
    object_list::add(&mut world, constant_medium::new(boundary, 0.2, Colour::new(0.2, 0.4, 0.9)));

    let boundary = sphere::new(Point3::zero(), 5000.0, Arc::new(Dialetric::new(1.5)));
    object_list::add(&mut world, constant_medium::new(boundary, 0.0001, Colour::from_value(1.0)));

    let emat = Arc::new(Lambertian::from_texture(Arc::new(ImageTexture::new("textures/earthmap.jpg"))));
    object_list::add(&mut world, sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    object_list::add(&mut world, sphere::new(Point3::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::from_texture(pertext))));

    let mut boxes2 = object_list::new();
    let white = Arc::new(Lambertian::new(Colour::from_value(0.73)));
    let ns = 1000;
    for _ in 0..ns {
        object_list::add(&mut boxes2, sphere::new(random_in_range(&mut rng, 0.0, 165.0), 10.0, white.clone()))
    }


    let mut world2 = object_list::new();
    let b = bvh::new(world, 0.0..1.0);
    object_list::add(&mut world2, b);

    world2
}
