use std::sync::Arc;

use jrpt::{
    colour::Colour,
    materials::{dialetric::Dialetric, lambertian::Lambertian, metal::Metal},
    objects::{
        affine::{self},
        bvh::{self},
        moving_sphere::{self},
        object_list::{self},
        sphere::{self},
        Object,
    },
    point3::Point3,
    random::{random, random_in_range},
    textures::checker_texture::CheckerTexture,
    vec3::Vec3,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub fn build_scene() -> Object {
    let mut world = object_list::new();

    let checker = Arc::new(CheckerTexture::new(
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::from_texture(checker));
    // let sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    let sphere = sphere::canonical(ground_material);
    let mut transform = affine::new(sphere);

    affine::scale_uniform(&mut transform, 1000.0);
    affine::translate(&mut transform, 0.0, -1000.0, 0.0);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, transform);

    let mut rng = SmallRng::seed_from_u64(1232);

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

            if (center.clone() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random(&mut rng) * random(&mut rng);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = &center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    let sphere =
                        moving_sphere::new(center, center2, 0.0..1.0, 0.2, sphere_material);
                    // let sphere = Sphere::new(center, 0.2, sphere_material);
                    object_list::add(&mut world, sphere);
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_in_range(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);

                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));

                    let sphere = sphere::canonical(sphere_material);
                    let mut transform = affine::new(sphere);
                    affine::scale_uniform(&mut transform, 0.2);
                    affine::translate(&mut transform, center.x, center.y, center.z);
                    affine::set_inverse(&mut transform);

                    // let sphere = Sphere::new(center, 0.2, sphere_material);
                    object_list::add(&mut world, transform);
                } else {
                    // glass
                    let sphere_material = Arc::new(Dialetric::new(1.5));
                    // let sphere = Sphere::new(center, 0.2, sphere_material);

                    let sphere = sphere::canonical(sphere_material);
                    let mut transform = affine::new(sphere);
                    affine::scale_uniform(&mut transform, 0.2);
                    affine::translate(&mut transform, center.x, center.y, center.z);
                    affine::set_inverse(&mut transform);

                    object_list::add(&mut world, transform);
                }
            }
        }
    }

    let material = Arc::new(Dialetric::new(1.5));
    // let sphere = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material);
    let sphere = sphere::canonical(material);
    let mut transform = affine::new(sphere);
    affine::translate(&mut transform, 0.0, 1.0, 0.0);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, transform);

    let material = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    // let sphere = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material);
    let sphere = sphere::canonical(material);
    let mut transform = affine::new(sphere);
    affine::translate(&mut transform, -4.0, 1.0, 0.0);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, transform);

    let material = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    // let sphere = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material);
    let sphere = sphere::canonical(material);
    let mut transform = affine::new(sphere);
    affine::translate(&mut transform, 4.0, 1.0, 0.0);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, transform);

    let mut world2 = object_list::new();
    let b = bvh::new(world, 0.0..1.0);
    object_list::add(&mut world2, b);

    world2
}
