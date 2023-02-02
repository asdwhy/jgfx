use std::{sync::Arc, f64::consts::PI};

use jgfxlib::{
    objects::{
        object_list::ObjectList, 
        aa_rectangles::{
            yz_rect::YzRectangle, 
            xz_rect::XzRectangle, 
            xy_rect::XyRectangle
        }, 
        rect_prism::RectangularPrism, 
        Object,
        affine::Affine, bvh::BvhNode, wavefront_obj::new_mesh, sphere::Sphere, constant_medium::ConstantMedium
    }, 
    materials::{
        lambertian::Lambertian, diffuse_light::DiffuseLight, metal::Metal, dialetric::Dialetric, Material
    }, 
    colour::Colour, point3::Point3, textures::{Texture, image_texture::ImageTexture, solid_colour::SolidColour}, random
};
use rand::{rngs::SmallRng, SeedableRng, Rng};

pub fn build_scene() -> ObjectList {
    let mut rng = SmallRng::seed_from_u64(1);
    let mut objects = ObjectList::new();

    // textures
    let lamp_texture = Arc::new(ImageTexture::new("textures/wood.jpg"));

    // materials
    let lamp_mat = Arc::new(Lambertian::from_texture(lamp_texture));
    let light_mat = Arc::new(DiffuseLight::new(Colour::new(1.0, 1.0, 0.8)));

    // lamp
    let obj = new_mesh("meshes/lamp3.obj".to_string(), lamp_mat);
    let b = BvhNode::new(obj, 0.0..0.0);
    let mut transform = Affine::new(Arc::new(b));
    transform.rotate_y(PI*0.2);
    transform.translate(5.0, -35.0, -25.0);
    transform.scale_uniform(0.5);
    transform.set_inverse();
    objects.add(Arc::new(transform));

    // light in lamp
    let sphere = Sphere::canonical(light_mat.clone());
    let mut transform = Affine::new(Arc::new(sphere));    
    transform.scale(1.8, 2.0, 1.75);
    transform.translate(5.2, -3.0, -10.1);
    transform.set_inverse();
    objects.add(Arc::new(transform));

    let sphere = Sphere::canonical(Arc::new(Dialetric::new(1.3)));
    let mut transform = Affine::new(Arc::new(sphere));    
    transform.scale(1.84, 2.04, 1.754);
    transform.translate(5.2, -3.0, -10.1);
    transform.set_inverse();
    objects.add(Arc::new(transform));

    // ceiling light
    let ceiling_height = 30.0;
    let rect_light = XzRectangle::new(-30.0, 30.0, 0.0, 60.0, ceiling_height * 0.999, light_mat.clone());
    objects.add(Arc::new(rect_light));
    
    // walls
    let left_wall = YzRectangle::new(-100.0, 100.0, -100.0, 300.0, 60.0, random_lambertian(&mut rng));
    objects.add(Arc::new(left_wall));

    let right_wall = YzRectangle::new(-100.0, 100.0, -100.0, 300.0, -60.0, random_lambertian(&mut rng));
    objects.add(Arc::new(right_wall));

    let top_wall = XzRectangle::new(-100.0, 100.0, -200.0, 300.0, ceiling_height, random_metal(&mut rng, 0.0));
    objects.add(Arc::new(top_wall));

    let back_wall = XyRectangle::new(-100.0, 100.0, -100.0, 300.0, 300.0, random_lambertian(&mut rng));
    objects.add(Arc::new(back_wall));

    let behind_wall = XyRectangle::new(-100.0, 100.0, -100.0, 300.0, -24.0, random_lambertian(&mut rng));
    // let behind_wall = XyRectangle::new(-100.0, 100.0, -200.0, 400.0, -24.0, light_mat.clone());
    objects.add(Arc::new(behind_wall));

    // ground
    for i in -4..4 {
        for j in -2..20 {
            let i = i as f64;
            let j = j as f64;

            let size = 15.0;

            let ground_texture = Arc::new(SolidColour::new(Colour::new(0.4, 0.3, 0.2)));
            let ground_mat = Arc::new(Metal::new(Colour::new(0.5, 0.4, 0.2), 0.0));
            let cube = RectangularPrism::canonical(ground_mat);
            let mut transform = Affine::new(Arc::new(cube));
            transform.scale_uniform(size*0.95);
            transform.translate(i*size, -2.0*size + rng.gen_range((-0.5*size)..(0.5*size)), j * size);
            transform.set_inverse();
            objects.add(Arc::new(transform));
        }
    }

    // constant medium
    let boundary = Arc::new(Sphere::new(Point3::zero(), 5000.0, Arc::new(Dialetric::new(1.5))));
    objects.add(Arc::new(ConstantMedium::new(boundary.clone(), 0.0005, Colour::from_value(1.0))));

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
            
            let sphere = Sphere::canonical(material);
            let mut transform = Affine::new(Arc::new(sphere));
            transform.scale_uniform(size* 0.3);
            transform.translate(i*size, size * 2.5 - 2.0*size + rng.gen_range(0.0..(size)), j * size);
            transform.set_inverse();
            objects.add(Arc::new(transform));
        }
    }

    let bvh = Arc::new(BvhNode::new(objects, 0.0..0.0));
    let mut world = ObjectList::new();
    world.add(bvh);

    world

}

fn random_lambertian(rng: &mut SmallRng) -> Arc<Lambertian> {
    Arc::new(Lambertian::new(jgfxlib::random::random(rng)))
}

fn random_metal(rng: &mut SmallRng, fuzzy: f64) -> Arc<Metal> {
    Arc::new(Metal::new(jgfxlib::random::random(rng), fuzzy))
}