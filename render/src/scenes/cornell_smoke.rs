// use std::sync::Arc;

// use jgfxlib::{
//     hittables::{
//         hittable_list::HittableList, 
//         aa_rectangles::{
//             yz_rect::YzRectangle, 
//             xz_rect::XzRectangle, 
//             xy_rect::XyRectangle
//         }, 
//         rect_prism::RectangularPrism, 
//         Hittable, 
//         bvh::BvhNode, 
//         constant_medium::ConstantMedium
//     }, 
//     materials::{
//         lambertian::Lambertian, diffuse_light::DiffuseLight
//     }, 
//     colour::Colour, point3::Point3
// };

// pub fn build_scene() -> HittableList {
//     let mut objects = HittableList::new();

//     let red = Arc::new(Lambertian::new(Colour::new(0.65, 0.05, 0.05)));
//     let white = Arc::new(Lambertian::new(Colour::new(0.73, 0.73, 0.73)));
//     let green = Arc::new(Lambertian::new(Colour::new(0.12, 0.45, 0.15)));
//     let light = Arc::new(DiffuseLight::new(Colour::from_value(7.0)));

//     // walls
//     objects.add(Arc::new(YzRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
//     objects.add(Arc::new(YzRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
//     objects.add(Arc::new(XzRectangle::new(113.0, 443.0, 127.0, 432.0, 554.0, light)));
//     objects.add(Arc::new(XzRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
//     objects.add(Arc::new(XzRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
//     objects.add(Arc::new(XyRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

//     // boxes
//     let mut b: Arc<dyn Hittable> = Arc::new(RectangularPrism::new(Point3::zero(), Point3::new(165.0, 330.0, 165.0), white.clone()));
//     // b = Arc::new(RotateY::new(b, 15.0));
//     // b = Arc::new(Translate::new(b, Point3::new(265.0, 0.0, 295.0)));
//     objects.add(Arc::new(ConstantMedium::new(b, 0.01, Colour::zero())));

//     let mut b: Arc<dyn Hittable> = Arc::new(RectangularPrism::new(Point3::zero(), Point3::from_value(165.0), white.clone()));
//     // b = Arc::new(RotateY::new(b, -18.0));
//     // b = Arc::new(Translate::new(b, Point3::new(130.0, 0.0, 65.0)));
//     objects.add(Arc::new(ConstantMedium::new(b, 0.01, Colour::from_value(1.0))));

//     // objects

//     let bvh = Arc::new(BvhNode::new(objects, 0.0..0.0));
//     let mut world = HittableList::new();
//     world.add(bvh);

//     world

// }
