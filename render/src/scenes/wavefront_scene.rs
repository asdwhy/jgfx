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
        affine::Affine, bvh::BvhNode, wavefront_obj::new_mesh
    }, 
    materials::{
        lambertian::Lambertian, diffuse_light::DiffuseLight, metal::Metal
    }, 
    colour::Colour, point3::Point3
};

pub fn build_scene() -> ObjectList {
    let mut objects = ObjectList::new();

    let red = Arc::new(Lambertian::new(Colour::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Colour::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Colour::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Colour::new(15.0, 15.0, 15.0)));

    // walls
    objects.add(Arc::new(YzRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XzRectangle::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    objects.add(Arc::new(XzRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.add(Arc::new(XzRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.add(Arc::new(XyRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    // boxes
    let b: Arc<dyn Object> = Arc::new(RectangularPrism::new(Point3::zero(), Point3::new(165.0, 330.0, 165.0), white.clone()));
    let mut transform = Affine::new(b);
    transform.rotate_y((15.0 as f64).to_radians());
    transform.translate(265.0, 0.0, 295.0);
    transform.set_inverse();
    objects.add(Arc::new(transform));

    // objects
    let monke_material = Arc::new(Metal::new(Colour::new(0.8, 0.4, 0.2), 0.5));
    let obj = new_mesh("meshes/monke.obj".to_string(), monke_material);
    let b = BvhNode::new(obj, 0.0..0.0);   
    // objects.add(Arc::new(b));

    let mut transform = Affine::new(Arc::new(b));
    transform.scale_uniform(100.0);
    transform.rotate_y(3.4*PI/4.0);
    transform.rotate_x(PI/5.0 * 0.99);
    transform.rotate_z(PI * 0.1);
    transform.rotate_y(-PI * 0.05);

    transform.translate(160.0, 42.0, 200.0);

    transform.set_inverse();
    objects.add(Arc::new(transform));

    let bvh = Arc::new(BvhNode::new(objects, 0.0..0.0));
    let mut world = ObjectList::new();
    world.add(bvh);

    world

}