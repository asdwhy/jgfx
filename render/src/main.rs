mod scenes;

use std::sync::Arc;

use jgfxlib::colour::Colour;
use jgfxlib::hittables::aa_rectangles::xz_rect::XzRectangle;
use jgfxlib::hittables::hittable_list::{HittableList};
use jgfxlib::hittables::sphere::Sphere;
use jgfxlib::materials::diffuse_light::DiffuseLight;
use jgfxlib::{camera::Camera};
use jgfxlib::vec3::Vec3;
use jgfxlib::scene::{Scene};
use jgfxlib::point3::Point3;
use jgfxlib::renderer::Renderer;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let now = std::time::Instant::now();

    // Image
    let aspect_ratio = 1.0;
    let image_width = 600 as u32;
    let samples_per_pixel = 1000;
    let max_depth = 30;

    // World
    let world: HittableList;
    let lookfrom: Point3;
    let lookat: Point3;
    let vfov: f64;
    let aperture: f64 = 0.0;
    let background_colour: Colour = Colour::zero();
    let time = 0.0..0.0;

    world = scenes::cornell_box::build_scene();
    let mut lights = HittableList::new();

    let light_mat = Arc::new(DiffuseLight::new(Colour::new(15.0, 15.0, 15.0)));
    lights.add(Arc::new(XzRectangle::new(213.0, 343.0, 227.0, 332.0, 554.0, light_mat.clone())));
    // lights.add(Arc::new(Sphere::new(Point3::new(190.0, 90.0, 190.0), 90.0, light_mat)));

    lookfrom = Point3::new(278.0, 278.0, -800.0);
    lookat = Point3::new(278.0, 278.0, 0.0);
    vfov = 40.0;

    let vup = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    
    let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, time);
    let scene = Scene::new(cam, world, Arc::new(lights), background_colour);

    // Render
    let mut renderer = Renderer::new();
    renderer.set_num_samples(samples_per_pixel);
    renderer.set_depth(max_depth);
    renderer.set_multithreading(true);

    let img = renderer.render(&scene, (image_width  as f64 / aspect_ratio) as u32, image_width);

    println!("Finished in {}", now.elapsed().as_secs_f64());
    
    println!("Writing to file...");
    
    img.save("image.png").unwrap();
}