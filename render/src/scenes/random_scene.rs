// use std::sync::Arc;

// use jgfxlib::{hittables::{hittable_list::HittableList, sphere::Sphere, moving_sphere::MovingSphere, bvh::BvhNode}, colour::Colour, textures::checker_texture::CheckerTexture, materials::{lambertian::Lambertian, metal::Metal, dialetric::Dialetric}, point3::Point3, vec3::Vec3};
// use rand::{rngs::SmallRng, SeedableRng, Rng};

// pub fn build_scene() -> HittableList {
//     let mut world = HittableList::new();

//     let checker = Arc::new(CheckerTexture::new(Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)));
//     let ground_material = Arc::new(Lambertian::from_texture(checker));
//     let sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
//     world.add(Arc::new(sphere));

//     let mut rng = SmallRng::seed_from_u64(1232);    

//     for a in -11..11 {
//         for b in -11..11 {
//             let a = a as f64;
//             let b = b as f64;

//             let choose_mat = rng.gen::<f64>();
//             let center = Point3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

//             if (center.clone() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
//                 if choose_mat < 0.8 {
//                     // diffuse
//                     let albedo = Colour::random(&mut rng) * Colour::random(&mut rng);
//                     let sphere_material = Arc::new(Lambertian::new(albedo));
//                     let center2 = &center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
//                     let sphere = MovingSphere::new(center, center2, 0.0..1.0, 0.2, sphere_material);
//                     // let sphere = Sphere::new(center, 0.2, sphere_material);
//                     world.add(Arc::new(sphere));
//                 } else if choose_mat < 0.95 {
//                     // metal
//                     let albedo = Colour::random_in_range(&mut rng, 0.5, 1.0);
//                     let fuzz = rng.gen_range(0.0..0.5); 

//                     let sphere_material = Arc::new(Metal::new(albedo, fuzz));
//                     let sphere = Sphere::new(center, 0.2, sphere_material);
//                     world.add(Arc::new(sphere));
//                 } else {
//                     // glass
//                     let sphere_material = Arc::new(Dialetric::new(1.5));
//                     let sphere = Sphere::new(center, 0.2, sphere_material);
//                     world.add(Arc::new(sphere));
//                 }
//             }
//         }
//     }


//     let material = Arc::new(Dialetric::new(1.5));
//     let sphere = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material);
//     world.add(Arc::new(sphere));

//     let material = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
//     let sphere = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material);
//     world.add(Arc::new(sphere));

//     let material = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
//     let sphere = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material);
//     world.add(Arc::new(sphere));

//     let mut world2 = HittableList::new();
//     let b = BvhNode::new(world, 0.0..1.0);
//     world2.add(Arc::new(b));
    
//     world2
// }
