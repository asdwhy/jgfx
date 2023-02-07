mod scenes;

use jgfxlib::colour::Colour;
use jgfxlib::objects::{Object, object_list};
use jgfxlib::{camera::Camera};
use jgfxlib::vec3::Vec3;
use jgfxlib::scene::{Scene};
use jgfxlib::point3::Point3;
use jgfxlib::renderer::Renderer;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let now = std::time::Instant::now();
    let multithreading = true;

    // Image
    let mut aspect_ratio = 16.0/9.0;
    let mut image_width = 512_u32;
    let mut samples_per_pixel = 300;
    let max_depth = 15;

    // World
    let world: Object;
    let lookfrom: Point3;
    let lookat: Point3;
    let vfov: f64;
    let aperture: f64 = 0.0;
    let mut background_colour: Colour = Colour::new(0.7, 0.8, 1.0);
    let time = 0.0..0.0;

    let scene_num = 5;

    match scene_num {
        1 => {
            world = scenes::two_spheres::build_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::zero();
            vfov = 20.0;
        },
        // 2 => {
        //     world = scenes::two_perlin_spheres::build_scene();
        //     lookfrom = Point3::new(13.0, 2.0, 3.0);
        //     lookat = Point3::zero();
        //     vfov = 20.0;
        // },
        // 3 => {
        //     world = scenes::earth::build_scene();
        //     lookfrom = Point3::new(0.0, 0.0, -15.0);
        //     lookat = Point3::zero();
        //     vfov = 20.0;
        // },
        // 4 => {
        //     world = scenes::simple_light::build_scene();
        //     lookfrom = Point3::new(26.0, 3.0, 6.0);
        //     lookat = Point3::new(0.0, 2.0, 0.0);
        //     vfov = 20.0;
        //     background_colour = Colour::zero();
        // },
        5 => {
            world = scenes::cornell_box::build_scene();
            aspect_ratio = 1.0;
            image_width = 400;
            samples_per_pixel = 200;
            background_colour = Colour::zero();

            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        },
        // 6 => {
        //     world = scenes::cornell_smoke::build_scene();
        //     aspect_ratio = 1.0;
        //     image_width = 400;
        //     samples_per_pixel = 400;
        //     background_colour = Colour::zero();

        //     lookfrom = Point3::new(278.0, 278.0, -800.0);
        //     lookat = Point3::new(278.0, 278.0, 0.0);
        //     vfov = 40.0;
        // },
        // 7 => {
        //     world = scenes::final_scene::build_scene();
        //     aspect_ratio = 1.0;
        //     image_width = 400;
        //     samples_per_pixel = 100;
        //     background_colour = Colour::zero();
        //     time = 0.0..1.0;
        //     lookfrom = Point3::new(478.0, 278.0, -600.0);
        //     lookat = Point3::new(278.0, 278.0, 0.0);
        //     vfov = 40.0;
        // },
        // 8 => {
        //     world = scenes::random_scene::build_scene();
        //     lookfrom = Point3::new(13.0, 2.0, 3.0);
        //     lookat = Point3::zero();
        //     vfov = 20.0;
        //     aperture = 0.1;
        //     time = 0.0..0.0;
        // },
        // 9 => {
        //     world = scenes::triangle_scene::build_scene();
        //     lookfrom = Point3::new(0.0, 0.0, -10.0);
        //     lookat = Point3::zero();
        //     vfov = 30.0;
        //     aperture = 0.0;
        //     time = 0.0..0.0;
        // },
        // 10 => {
        //     world = scenes::wavefront_scene::build_scene();
        //     aspect_ratio = 1.0;
        //     image_width = 1024;
        //     samples_per_pixel = 10000;
        //     background_colour = Colour::zero();
        //     lookat = Point3::new(278.0, 278.0, 0.0);
        //     lookfrom = Point3::new(278.0, 278.0, -800.0);
        //     vfov = 40.0;
        //     aperture = 0.0;
        //     time = 0.0..0.0;
        // },
        // 11 => {
        //     world = scenes::scene1::build_scene();
        //     image_width = 2048;
        //     samples_per_pixel = 20000;
        //     background_colour = Colour::zero();
        //     lookfrom = Point3::new(50.0,5.0,-20.0);
        //     lookat = Point3::new(0.0, 0.0, 50.0);
        //     vfov = 90.0;
        //     aperture = 0.0;
        //     time = 0.0..0.0;
        // },
        _ => {
            world = object_list::new();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::zero();
            vfov = 20.0;
            background_colour = Colour::zero();
        }
    }

    let vup = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    
    let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, time);
    let scene = Scene::new(cam, world, background_colour);

    // Render
    let renderer = Renderer::new(samples_per_pixel, max_depth, multithreading);

    let img = renderer.render(&scene, (image_width  as f64 / aspect_ratio) as u32, image_width);

    println!("Finished in {}", now.elapsed().as_secs_f64());
    
    println!("Writing to file...");
    
    img.save("image.png").unwrap();
}