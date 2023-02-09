use image::{RgbImage, DynamicImage};
use crate::{
    colour::Colour,
    point3::Point3,
    textures::Texture,
    utils::clamp
};

pub struct ImageTexture {
    data: RgbImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let img = match image::open(filename) {
            Err(err) => {
                eprintln!("Error opening {filename}: {err}");
                panic!();
            },
            Ok(dynamic_image) => {
                match dynamic_image {
                    DynamicImage::ImageRgb8(image) => image, 
                    _ => {
                        eprintln!("Can not create image texture from non-rgb image!");
                        panic!();
                    }
                }
            }
        };

        Self {
            data: img
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Point3) -> Colour {
        // clamp u,v to [0,1]
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0); // flip v to image coordinates

        let mut i = (u * self.data.width() as f64) as u32;
        let mut j = (v * self.data.height() as f64) as u32;

        if i >= self.data.width() {
            i = self.data.width() - 1;
        }

        if j >= self.data.height() {
            j = self.data.height() - 1;
        }

        let colour_scale = 1.0/255.0;
        let pixel = (self.data.get_pixel(i,j)).0;
        
        Colour::new(colour_scale * pixel[0] as f64, colour_scale * pixel[1] as f64, colour_scale * pixel[2] as f64)
    }
}