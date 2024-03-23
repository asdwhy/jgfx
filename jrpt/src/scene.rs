use crate::{
    colour::Colour,
    objects::Object,
    camera::Camera
};

pub struct Scene {
    pub objects: Object,
    pub camera: Camera,
    pub background_colour: Colour
}

impl Scene {
    pub fn new(camera: Camera, objects: Object, background_colour: Colour) -> Self {
        Self {
            camera,
            objects,
            background_colour
        }
    }

    /// Set background colour for this render
    pub fn set_background_colour(&mut self, colour: Colour) {
        self.background_colour = colour;
    }
}


