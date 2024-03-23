use std::{sync::Arc, f64::consts::PI};

use jrpt::{
    objects::{
        object_list::{ObjectList, self},
        aa_rectangles::{
            yz_rect::{YzRectangle, self},
            xz_rect::{XzRectangle, self},
            xy_rect::{XyRectangle, self}
        }, 
        rect_prism::{RectangularPrism, self},
        Object,
        affine::{Affine, self}, bvh::{BvhNode, self}, wavefront_obj::new_mesh, sphere::{Sphere, self}, constant_medium::{ConstantMedium, self}
    }, 
    materials::{
        lambertian::Lambertian, diffuse_light::DiffuseLight, metal::Metal, dialetric::Dialetric, Material
    }, 
    colour::Colour, point3::Point3, textures::{Texture, image_texture::ImageTexture, solid_colour::SolidColour}, random
};
use rand::{rngs::SmallRng, SeedableRng, Rng};

pub fn build_scene() -> Object {
    let mut rng = SmallRng::seed_from_u64(1);
    let mut world = object_list::new();

    // textures
    let lamp_texture = Arc::new(ImageTexture::new("textures/wood.jpg"));

    // materials
    let lamp_mat = Arc::new(Lambertian::from_texture(lamp_texture));
    let light_mat = Arc::new(DiffuseLight::new(Colour::new(1.0, 1.0, 0.8)));

    // lamp
    let obj = new_mesh("meshes/lamp3.obj".to_string(), lamp_mat);
    let b = bvh::new(obj, 0.0..0.0);
    let mut transform = affine::new(b);
    affine::rotate_y(&mut transform, PI*0.2);
    affine::translate(&mut transform, 5.0, -35.0, -25.0);
    affine::scale_uniform(&mut transform, 0.5);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, transform);

    // light in lamp
    let sphere = sphere::canonical(light_mat.clone());
    let mut transform = affine::new(sphere);
    affine::scale(&mut transform, 1.8, 2.0, 1.75);
    affine::translate(&mut transform, 5.2, -3.0, -10.1);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, transform);

    let sphere = sphere::canonical(Arc::new(Dialetric::new(1.3)));
    let mut transform = affine::new(sphere);
    affine::scale(&mut transform, 1.84, 2.04, 1.754);
    affine::translate(&mut transform, 5.2, -3.0, -10.1);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, transform);

    // ceiling light
    let ceiling_height = 30.0;
    let rect_light = xz_rect::new(-30.0, 30.0, 0.0, 60.0, ceiling_height * 0.999, light_mat.clone());
    object_list::add(&mut world, rect_light);

    // walls
    let left_wall = yz_rect::new(-100.0, 100.0, -100.0, 300.0, 60.0, random_lambertian(&mut rng));
    object_list::add(&mut world, left_wall);

    let right_wall = yz_rect::new(-100.0, 100.0, -100.0, 300.0, -60.0, random_lambertian(&mut rng));
    object_list::add(&mut world, right_wall);

    let top_wall = xz_rect::new(-100.0, 100.0, -200.0, 300.0, ceiling_height, random_metal(&mut rng, 0.0));
    object_list::add(&mut world, top_wall);

    let back_wall = xy_rect::new(-100.0, 100.0, -100.0, 300.0, 300.0, random_lambertian(&mut rng));
    object_list::add(&mut world, back_wall);

    let behind_wall = xy_rect::new(-100.0, 100.0, -100.0, 300.0, -24.0, random_lambertian(&mut rng));
    // let behind_wall = XyRectangle::new(-100.0, 100.0, -200.0, 400.0, -24.0, light_mat.clone());
    object_list::add(&mut world, behind_wall);

    // ground
    for i in -4..4 {
        for j in -2..20 {
            let i = i as f64;
            let j = j as f64;

            let size = 15.0;

            let ground_texture = Arc::new(SolidColour::new(Colour::new(0.4, 0.3, 0.2)));
            let ground_mat = Arc::new(Metal::new(Colour::new(0.5, 0.4, 0.2), 0.0));
            let cube = rect_prism::canonical(ground_mat);
            let mut transform = affine::new(cube);
            affine::scale_uniform(&mut transform, size*0.95);
            affine::translate(&mut transform, i*size, -2.0*size + rng.gen_range((-0.5*size)..(0.5*size)), j * size);
            affine::set_inverse(&mut transform);
            object_list::add(&mut world, transform);
        }
    }

    // constant medium
    let boundary = sphere::new(Point3::zero(), 5000.0, Arc::new(Dialetric::new(1.5)));
    object_list::add(&mut world, constant_medium::new(boundary, 0.0005, Colour::from_value(1.0)));

    // spheres
    for i in -4..4 {
        for j in 0..20 {
            let i = i as f64;
            let j = j as f64;

            if rng.gen::<f64>() >= 0.31 {
                continue;
            }

            let size = 15.0;
            let num = rng.gen_range(0.0..1.0);

            let material: Arc<dyn Material> = if num <= 0.4 { // metal
                Arc::new(Metal::new(random::random(&mut rng), rng.gen_range(0.0..1.0)))
            } else if num <= 0.8 {
                Arc::new(Dialetric::new(rng.gen_range(1.2..2.4)))
            } else {
                random_lambertian(&mut rng)
            };
            
            let sphere = sphere::canonical(material);
            let mut transform = affine::new(sphere);
            affine::scale_uniform(&mut transform, size* 0.3);
            affine::translate(&mut transform, i*size, size * 2.5 - 2.0*size + rng.gen_range(0.0..(size)), j * size);
            affine::set_inverse(&mut transform);
            object_list::add(&mut world, transform);
        }
    }

    let bvh = bvh::new(world, 0.0..0.0);
    let mut world = object_list::new();
    object_list::add(&mut world, bvh);

    world
}

fn random_lambertian(rng: &mut SmallRng) -> Arc<Lambertian> {
    Arc::new(Lambertian::new(jrpt::random::random(rng)))
}

fn random_metal(rng: &mut SmallRng, fuzzy: f64) -> Arc<Metal> {
    Arc::new(Metal::new(jrpt::random::random(rng), fuzzy))
}
