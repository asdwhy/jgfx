pub mod vec3;
pub mod constants;
pub mod utils;
pub mod colour;
pub mod ray;
pub mod camera;
pub mod hittables;
pub mod scene;
pub mod renderer;
pub mod materials;

pub mod point {
    pub type Point3 = crate::vec3::Vec3;
}

#[cfg(test)]
pub mod test;