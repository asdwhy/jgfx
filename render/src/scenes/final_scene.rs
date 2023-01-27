// use std::sync::Arc;
// use rand::{rngs::SmallRng, SeedableRng, Rng};
// use jgfxlib::{
//     hittables::{
//         hittable_list::ObjectList, aa_rectangles::{
//             xz_rect::XzRectangle,
//         }, 
//         rect_prism::RectangularPrism,  
//         bvh::BvhNode, 
//         constant_medium::ConstantMedium, 
//         moving_sphere::MovingSphere,
//         sphere::Sphere
//     }, 
//     materials::{
//         lambertian::Lambertian, 
//         diffuse_light::DiffuseLight,
//         dialetric::Dialetric, metal::Metal
//     }, 
//     colour::Colour, 
//     point3::Point3, 
//     textures::{image_texture::ImageTexture, noise_texture::NoiseTexture}, 
//     vec3::Vec3, 
//     random::random_in_range
// };

// const BOXES_PER_SIDE: i32 = 20;

// pub fn build_scene() -> ObjectList {
//     let mut boxes1 = ObjectList::new();
//     let ground_mat = Arc::new(Lambertian::new(Colour::new(0.48, 0.83, 0.53)));

//     let mut rng = SmallRng::seed_from_u64(000000);

//     for i in 0..BOXES_PER_SIDE {
//         for j in 0..BOXES_PER_SIDE {
//             let w = 100.0;
//             let x0 = -1000.0 + (i as f64)*w;
//             let z0 = -1000.0 + (j as f64)*w;
//             let y0 = 0.0;
//             let x1 = x0 + w;
//             let y1 = rng.gen_range(1.0..101.0);
//             let z1 = z0 + w;

//             boxes1.add(Arc::new(RectangularPrism::new(Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), ground_mat.clone())))
//         }
//     }

//     let mut objects = ObjectList::new();
//     objects.add(Arc::new(BvhNode::new(boxes1, 0.0..1.0)));

//     let light = Arc::new(DiffuseLight::new(Colour::from_value(7.0)));
//     objects.add(Arc::new(XzRectangle::new(123.0,423.0,147.0,412.0,554.0, light.clone())));

//     let center1 = Point3::new(400.0,400.0,200.0);
//     let center2 = &center1 + Point3::new(30.0,0.0,0.0);
//     let moving_sphere_material = Arc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.1)));
//     objects.add(Arc::new(MovingSphere::new(center1, center2, 0.0..1.0, 50.0, moving_sphere_material.clone())));

//     objects.add(Arc::new(Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, Arc::new(Dialetric::new(1.5)))));
//     objects.add(Arc::new(Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(Colour::new(0.8, 0.8, 0.9), 1.0)))));

//     let boundary = Arc::new(Sphere::new(Point3::new(360.0,150.0,145.0), 70.0, Arc::new(Dialetric::new(1.5))));
//     objects.add(boundary.clone());
//     objects.add(Arc::new(ConstantMedium::new(boundary.clone(), 0.2, Colour::new(0.2, 0.4, 0.9))));

//     let boundary = Arc::new(Sphere::new(Point3::zero(), 5000.0, Arc::new(Dialetric::new(1.5))));
//     objects.add(Arc::new(ConstantMedium::new(boundary.clone(), 0.0001, Colour::from_value(1.0))));

//     let emat = Arc::new(Lambertian::from_texture(Arc::new(ImageTexture::new("textures/earthmap.jpg"))));
//     objects.add(Arc::new(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat.clone())));
//     let pertext = Arc::new(NoiseTexture::new(0.1));
//     objects.add(Arc::new(Sphere::new(Point3::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::from_texture(pertext)))));


//     let mut boxes2 = ObjectList::new();
//     let white = Arc::new(Lambertian::new(Colour::from_value(0.73)));
//     let ns = 1000;
//     for _ in 0..ns {
//         boxes2.add(Arc::new(Sphere::new(random_in_range(&mut rng, 0.0, 165.0), 10.0, white.clone())));
//     }

//     // objects.add(Arc::new(
//     //     Translate::new(
//     //         Arc::new(RotateY::new(Arc::new(BvhNode::new(boxes2, 0.0..1.0)), 15.0)),
//     //         Vec3::new(-100.0, 270.0, 395.0)
//     //     )
//     // ));

//     objects
// }
