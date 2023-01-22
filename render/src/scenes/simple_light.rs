// use std::sync::Arc;

// use jgfxlib::{
//     hittables::{
//         hittable_list::HittableList, sphere::Sphere, aa_rectangles::xy_rect::XyRectangle
//     }, 
//     textures::{noise_texture::NoiseTexture}, 
//     materials::{
//         lambertian::Lambertian, diffuse_light::DiffuseLight
//     }, 
//     point3::Point3, colour::Colour
// };

// pub fn build_scene() -> HittableList {
//     let mut world = HittableList::new();

//     let perlin_texture = Arc::new(NoiseTexture::new(4.0));
//     let permat = Arc::new(Lambertian::from_texture(perlin_texture));

//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, -1000.0, 0.0), 
//         1000.0,
//         permat.clone()))
//     );

//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, 2.0, 0.0), 
//         2.0,
//         permat.clone()))
//     );

//     let difflight = Arc::new(DiffuseLight::new(Colour::from_value(4.0)));
//     world.add(Arc::new(XyRectangle::new(
//         3.0, 
//         5.0, 
//         1.0, 
//         3.0, 
//         -2.0, 
//         difflight))
//     );


//     world
// }
