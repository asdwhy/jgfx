use std::sync::Arc;

use jgfxlib::{objects::{object_list::ObjectList, sphere::Sphere, moving_sphere::MovingSphere, bvh::BvhNode, affine::Affine}, colour::Colour, textures::checker_texture::CheckerTexture, materials::{lambertian::Lambertian, metal::Metal, dialetric::Dialetric}, point3::Point3, vec3::Vec3, random::{random, random_in_range}};
use rand::{rngs::SmallRng, SeedableRng, Rng};

pub fn build_scene() -> ObjectList {
    let mut world = ObjectList::new();

    let checker = Arc::new(CheckerTexture::new(Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)));
    let ground_material = Arc::new(Lambertian::from_texture(checker));
    // let sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    let sphere = Sphere::canonical(ground_material);
    let mut transform = Affine::new(Arc::new(sphere));
    transform.scale_uniform(1000.0);
    transform.translate(0.0, -1000.0, 0.0);
    transform.set_inverse();
    world.add(Arc::new(transform));

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
                    let sphere = MovingSphere::new(center, center2, 0.0..1.0, 0.2, sphere_material);
                    // let sphere = Sphere::new(center, 0.2, sphere_material);
                    world.add(Arc::new(sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_in_range(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5); 

                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));

                    let sphere = Sphere::canonical(sphere_material);
                    let mut transform = Affine::new(Arc::new(sphere));
                    transform.scale_uniform(0.2);
                    transform.translate(center.x, center.y, center.z);
                    transform.set_inverse();

                    // let sphere = Sphere::new(center, 0.2, sphere_material);
                    world.add(Arc::new(transform));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dialetric::new(1.5));
                    // let sphere = Sphere::new(center, 0.2, sphere_material);

                    let sphere = Sphere::canonical(sphere_material);
                    let mut transform = Affine::new(Arc::new(sphere));
                    transform.scale_uniform(0.2);
                    transform.translate(center.x, center.y, center.z);
                    transform.set_inverse();

                    world.add(Arc::new(transform));
                }
            }
        }
    }


    let material = Arc::new(Dialetric::new(1.5));
    // let sphere = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material);
    let sphere = Sphere::canonical(material);
    let mut transform = Affine::new(Arc::new(sphere));
    transform.translate(0.0, 1.0, 0.0);
    transform.set_inverse();
    world.add(Arc::new(transform));

    let material = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    // let sphere = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material);
    let sphere = Sphere::canonical(material);
    let mut transform = Affine::new(Arc::new(sphere));
    transform.translate(-4.0, 1.0, 0.0);
    transform.set_inverse();
    world.add(Arc::new(transform));

    let material = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    // let sphere = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material);
    let sphere = Sphere::canonical(material);
    let mut transform = Affine::new(Arc::new(sphere));
    transform.translate(4.0, 1.0, 0.0);
    transform.set_inverse();
    world.add(Arc::new(transform));

    let mut world2 = ObjectList::new();
    let b = BvhNode::new(world, 0.0..1.0);
    world2.add(Arc::new(b));
    
    world2

    // world
}
