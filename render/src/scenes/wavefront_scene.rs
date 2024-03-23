use std::{f64::consts::PI, sync::Arc};

use jrpt::{
    colour::Colour,
    materials::{diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal},
    objects::{
        aa_rectangles::{
            xy_rect::{self},
            xz_rect::{self},
            yz_rect::{self},
        },
        affine::{self},
        bvh::{self},
        object_list::{self},
        rect_prism::{self},
        wavefront_obj::new_mesh,
        Object,
    },
    point3::Point3,
};

pub fn build_scene() -> Object {
    let mut world = object_list::new();

    let red = Arc::new(Lambertian::new(Colour::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Colour::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Colour::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Colour::new(15.0, 15.0, 15.0)));

    // walls
    object_list::add(
        &mut world,
        yz_rect::new(0.0, 555.0, 0.0, 555.0, 555.0, green),
    );
    object_list::add(&mut world, yz_rect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));
    object_list::add(
        &mut world,
        xz_rect::new(213.0, 343.0, 227.0, 332.0, 554.0, light),
    );
    object_list::add(
        &mut world,
        xz_rect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()),
    );
    object_list::add(
        &mut world,
        xz_rect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()),
    );
    object_list::add(
        &mut world,
        xy_rect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()),
    );

    // boxes
    let b = rect_prism::new(
        Point3::zero(),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let mut transform = affine::new(b);
    affine::rotate_y(&mut transform, (15.0 as f64).to_radians());
    affine::translate(&mut transform, 265.0, 0.0, 295.0);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, transform);

    // objects
    let monke_material = Arc::new(Metal::new(Colour::new(0.8, 0.4, 0.2), 0.5));
    let obj = new_mesh("meshes/monke.obj".to_string(), monke_material);
    let b = bvh::new(obj, 0.0..0.0);

    let mut transform = affine::new(b);
    affine::scale_uniform(&mut transform, 100.0);
    affine::rotate_y(&mut transform, 3.4 * PI / 4.0);
    affine::rotate_x(&mut transform, PI / 5.0 * 0.99);
    affine::rotate_z(&mut transform, PI * 0.1);
    affine::rotate_y(&mut transform, -PI * 0.05);
    affine::translate(&mut transform, 160.0, 42.0, 200.0);
    affine::set_inverse(&mut transform);

    object_list::add(&mut world, transform);

    let bvh = bvh::new(world, 0.0..0.0);
    let mut world = object_list::new();
    object_list::add(&mut world, bvh);

    world
}
