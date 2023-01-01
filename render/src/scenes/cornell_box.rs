use std::sync::Arc;

use jgfxlib::{
    hittables::{
        hittable_list::HittableList, aa_rectangles::{yz_rect::YzRectangle, xz_rect::XzRectangle, xy_rect::XyRectangle}, rect_prism::RectangularPrism, translate::Translate, Hittable, rotate_y::RotateY, bvh::BvhNode, flip_face::FlipFace, sphere::Sphere
    }, 
    materials::{
        lambertian::Lambertian, diffuse_light::DiffuseLight, metal::Metal, dialetric::Dialetric
    }, 
    colour::Colour, point3::Point3
};

pub fn build_scene() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new(Colour::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Colour::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Colour::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Colour::new(15.0, 15.0, 15.0)));

    // walls
    objects.add(Arc::new(YzRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    
    // light
    let light_obj = Arc::new(FlipFace::new(Arc::new(XzRectangle::new(213.0, 343.0, 227.0, 332.0, 554.0, light.clone()))));
    objects.add(light_obj.clone());

    objects.add(Arc::new(XzRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.add(Arc::new(XzRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.add(Arc::new(XyRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    // boxes
    let aluminum = Arc::new(Metal::new(Colour::new(0.8, 0.85, 0.88), 0.0));
    let mut b: Arc<dyn Hittable> = Arc::new(RectangularPrism::new(Point3::new(265.0, 100.0, 295.0), Point3::new(165.0+265.0, 330.0, 165.0 + 265.0), aluminum.clone()));
    // let mut b: Arc<dyn Hittable> = Arc::new(RectangularPrism::new(Point3::zero(), Point3::new(165.0, 330.0, 165.0), aluminum.clone()));
    // b = Arc::new(RotateY::new(b, 15.0));
    // b = Arc::new(Translate::new(b, Point3::new(265.0, 0.0, 295.0)));
    objects.add(b);

    // closer box
    let mut b: Arc<dyn Hittable> = Arc::new(RectangularPrism::new(Point3::zero(), Point3::from_value(165.0), white.clone()));
    // b = Arc::new(RotateY::new(b, -18.0));
    // b = Arc::new(Translate::new(b, Point3::new(130.0, 0.0, 65.0)));
    // objects.add(b);

    // let glass = Arc::new(Dialetric::new(1.5));
    // objects.add(Arc::new(Sphere::new(Point3::new(190.0, 90.0, 190.0), 90.0, aluminum)));

    // objects

    let bvh = Arc::new(BvhNode::new(objects, 0.0..0.0));
    let mut world = HittableList::new();
    world.add(bvh);

    world

}
