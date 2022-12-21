use image::buffer::EnumeratePixelsMut;
use image::{ImageBuffer, RgbImage, Rgb};
use rand::{SeedableRng, Rng, thread_rng};
use rand::rngs::SmallRng;
use rayon::prelude::*;

use crate::scene::Scene;
use crate::ray::Ray;
use crate::hittables::{Hittable}; 
use crate::constants::{INFINITY, EPSILON};
use crate::utils::max;
use crate::colour::{Colour};

pub struct Renderer {
    num_samples: u32,
    depth: u32,
    multithreading: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            num_samples: 10,
            depth: 10,
            multithreading: false,
        }
    }

    /// Set image to take val samples
    /// Will sample minimum of one time
    pub fn set_num_samples(&mut self, num_samples: u32) {
        self.num_samples = max(1, num_samples);
    }

    /// Set the recursion depth
    pub fn set_depth(&mut self, val: u32) {
        self.depth = max(1, val);
    }

    /// Allow this render to be multithreaded
    pub fn set_multithreading(&mut self, val: bool) {
        self.multithreading = val;
    }

    pub fn render(&mut self, scene: &Scene, image_height: u32, image_width: u32) -> RgbImage {
        let mut img = ImageBuffer::new(image_width, image_height);

        let f = |(_, cols): (u32, EnumeratePixelsMut<Rgb<u8>>)| {
            let thread_rng = thread_rng();
            let mut rng = SmallRng::from_rng(thread_rng).unwrap();    
            
            cols.for_each(|(i, j, pixel): (u32, u32, &mut Rgb<u8>)| {
                let j = image_height - j;
                        // because from top down
                
                let col = self.sample_pixel(&mut rng, scene, i, j, image_height, image_width);

                *pixel = col.to_rgb();
            });
        };

        if self.multithreading {
            img.enumerate_rows_mut().par_bridge().for_each(f);
        } else {
            img.enumerate_rows_mut().for_each(f);
        }

        img
    }

    /// Antialias num_samples times on pixel (i,j)
    fn sample_pixel(&self, rng: &mut SmallRng, scene: &Scene, i: u32, j: u32, height: u32, width: u32) -> Colour {
        let mut col = Colour::zero();

        (0..self.num_samples).for_each(|_| {
            let u_ = ((i as f64) + rng.gen::<f64>()) / (width - 1) as f64;
            let v_ = ((j as f64) + rng.gen::<f64>()) / (height - 1) as f64;

            let r = scene.camera.get_ray(rng, u_, v_);
            col += self.path_trace(rng, scene, r, self.depth);
        });

        col / self.num_samples as f64
    }

    fn path_trace(&self, rng: &mut SmallRng, scene: &Scene, r: Ray, depth: u32) -> Colour {
        // max recursion limit reached
        if depth <= 0 {
            return Colour::zero();
        }

        let hit = scene.objects.intersect(&r, EPSILON, INFINITY);

        match hit {
            None => {
                let unit_dir = r.dir.normalized();
    
                let t = 0.5 * (unit_dir.y + 1.0);
                
                (1.0 - t) * Colour::new(1.0,1.0,1.0) + t*Colour::new(0.5, 0.7, 1.0)
            },
            Some(rec) => {
                match rec.material.scatter(rng, r, &rec) {
                    Some((attenuation, scattered)) => {
                        attenuation * self.path_trace(rng, scene, scattered, depth - 1)
                    },
                    None => Colour::zero()
                }
            }
        }
    }
}
