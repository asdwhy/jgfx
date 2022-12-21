use std::{sync::Arc};

use jgfxlib::colour::Colour;
use jgfxlib::hittables::hittable_list::HittableList;
use jgfxlib::hittables::objects::Object;
use jgfxlib::materials::dialetric::Dialetric;
use jgfxlib::materials::lambertian::{Lambertian};
use jgfxlib::materials::metal::Metal;
use jgfxlib::random::{random_f64, random_f64_range};
use jgfxlib::{camera::Camera};
use jgfxlib::vec3::Vec3;
use jgfxlib::hittables::objects::sphere::Sphere;
use jgfxlib::scene::Scene;
use jgfxlib::point::Point3;
use jgfxlib::renderer::Renderer;

fn main() {
    let now = std::time::Instant::now();
    
    // Image
    let aspect_ratio = 3.0/2.0;
    let image_width = 400 as u32;
    let image_height = (image_width  as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 15;
    let max_depth = 25;

    // World objects
    let objects = build_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let up = Vec3::new(0.0,1.0,0.0);
    let aperture = 0.2;
    let dist_to_focus = 10.0;

    let cam = Camera::new(lookfrom, lookat, up, 20.0, aspect_ratio, aperture, dist_to_focus);

    // Scene
    let scene = Scene::new(cam, objects);

    // Render
    let mut renderer = Renderer::new();
    renderer.set_antialiasing(samples_per_pixel);
    renderer.set_debug(false);
    renderer.set_depth(max_depth);
    renderer.set_multithreading(true);

    let img = renderer.render(&scene, image_height, image_width);

    println!("Finished in {}", now.elapsed().as_secs_f64());
    
    println!("Writing to file...");
    
    img.save("image.png").unwrap();

    // Sphere::ne
}


fn build_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Colour::new(0.5,0.5,0.5)));
    let mut sphere = Sphere::new(ground_material);
    sphere.set_origin(Vec3::new(0.0, -1000.0, 0.0));
    sphere.set_radius(1000.0);
    world.add(Arc::new(sphere));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat = random_f64();
            let center = Point3::new(a + 0.9 * random_f64(), 0.2, b + 0.9 * random_f64());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Colour::random() * Colour::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let mut sphere = Sphere::new(sphere_material);
                    sphere.set_origin(center);
                    sphere.set_radius(0.2);
                    world.add(Arc::new(sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::random_in_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);

                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    let mut sphere = Sphere::new(sphere_material);
                    sphere.set_origin(center);
                    sphere.set_radius(0.2);
                    world.add(Arc::new(sphere));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dialetric::new(1.5));
                    let mut sphere = Sphere::new(sphere_material);
                    sphere.set_origin(center);
                    sphere.set_radius(0.2);
                    world.add(Arc::new(sphere));
                }
            }
        }
    }


    let material = Arc::new(Dialetric::new(1.5));
    let mut sphere = Sphere::new(material);
    sphere.set_origin(Point3::new(0.0, 1.0, 0.0));
    sphere.set_radius(1.0);
    world.add(Arc::new(sphere));

    let material = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    let mut sphere = Sphere::new(material);
    sphere.set_origin(Point3::new(-4.0, 1.0, 0.0));
    sphere.set_radius(1.0);
    world.add(Arc::new(sphere));

    let material = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    let mut sphere = Sphere::new(material);
    sphere.set_origin(Point3::new(4.0, 1.0, 0.0));
    sphere.set_radius(1.0);
    world.add(Arc::new(sphere));

    world
}