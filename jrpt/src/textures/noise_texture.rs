use crate::{
    colour::Colour,
    point3::Point3,
    textures::{perlin::Perlin, Texture}
};

pub struct NoiseTexture {
    noise: Perlin,
    frequency: f64
}

impl NoiseTexture {
    pub fn new(frequency: f64) -> Self {
        Self {
            noise: Perlin::new(),
            frequency
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &Point3) -> Colour {
        let mut noise = self.frequency * p.z + 10.0 * self.noise.turbulence(p, 7);
        noise = noise.sin();
        noise = (1.0 + noise) * 0.5; // since noise in [-1, 1], map it to valid colours

        Colour::from_value(1.0) * noise
    }
}
