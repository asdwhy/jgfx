use crate::hittables::{hittable_list::HittableList};
use crate::camera::Camera;

pub struct Scene {
    pub objects: HittableList,
    pub camera: Camera
}


impl Scene {
    pub fn new(camera: Camera, objects: HittableList) -> Self {
        Self {
            camera,
            objects
        }
    }
}


