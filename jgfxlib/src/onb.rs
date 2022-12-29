// orthonormal basis class

use crate::vec3::Vec3;

pub struct Onb {
    axis: [Vec3; 3]
}

impl Onb {
    pub fn new(n: &Vec3) -> Self {
        let a2 = n.normalized();
        let a = if (a2.x).abs() > 0.9 { Vec3::new(0.0,1.0,0.0) } else { Vec3::new(1.0,0.0,0.0) };
        let a1 = a2.cross(&a).normalized();
        let a0 = a2.cross(&a1);

        Self {
            axis: [a0, a1, a2]
        }
    }

    pub fn u(&self) -> &Vec3 {
        &self.axis[0]
    }

    pub fn v(&self) -> &Vec3 {
        &self.axis[1]
    }

    pub fn w(&self) -> &Vec3 {
        &self.axis[2]
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a* self.u() + b*self.v() + c*self.w()
    }

    pub fn local_vec(&self, v: &Vec3) -> Vec3 {
        v.x*self.u() + v.y*self.v() + v.z*self.w()
    }
}