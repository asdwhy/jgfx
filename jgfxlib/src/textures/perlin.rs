use rand::{rngs::SmallRng, SeedableRng};
use crate::{
    random::{random_i32, random_in_range}, 
    point3::Point3, 
    vec3::Vec3
};

/// Generates perlin noise
const POINT_COUNT: usize = 256;

pub struct Perlin {
    random_vectors: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT]
}

impl Perlin {
    pub fn new() -> Self {
        let mut random_vectors: [Vec3; POINT_COUNT] = [Vec3::zero(); POINT_COUNT];
        let mut rng = SmallRng::from_entropy();

        for i in 0..POINT_COUNT {
            random_vectors[i] = random_in_range(&mut rng, -1.0, 1.0);
        }
        
        Self {
            random_vectors,
            perm_x: Self::generate_perlin_permutation(),
            perm_y: Self::generate_perlin_permutation(),
            perm_z: Self::generate_perlin_permutation()
        }
    }

    /// Returns noise in [-1,1]
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let init = Vec3::zero();
        let mut c: [[[&Vec3; 2]; 2]; 2] = [[[&init; 2]; 2]; 2];

        for di in 0i32..2i32 {
            for dj in 0i32..2i32 {
                for dk in 0i32..2i32 {
                    c[di as usize][dj as usize][dk as usize] = self.random_vectors.get(
                        self.perm_x[((i+di) & 255) as usize] as usize ^
                        self.perm_y[((j+dj) & 255) as usize] as usize ^
                        self.perm_z[((k+dk) & 255) as usize] as usize
                    ).unwrap();
                }
            }
        }

        Self::trilinear_interpolate(c, u, v, w)
    }

    /// Returns turbulent noise
    pub fn turbulence(&self, p: &Point3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut tmp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * Self::noise(&self, &tmp_p);
            weight *= 0.5;
            tmp_p *= 2.0;
        }

        accum.abs()
    }

    fn generate_perlin_permutation() -> [i32; POINT_COUNT] {
        let mut p: [i32; POINT_COUNT] = [0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }

        Self::permute(&mut p);

        p
    }

    fn permute(p: &mut[i32; POINT_COUNT]) {
        for i in (0..POINT_COUNT-1).rev() {
            let target = random_i32(0..((i+1usize) as i32)) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    fn trilinear_interpolate(c: [[[&Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Hermite cubic to give smoother interpolation
        let uu = u * u * (3.0-2.0*u);
        let vv = v * v * (3.0-2.0*v);
        let ww = w * w * (3.0-2.0*w);

        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let if64 = i as f64;
                    let jf64 = j as f64;
                    let kf64 = k as f64;

                    let weight = Vec3::new(u-if64, v-jf64, w-kf64);

                    accum +=    (if64*uu + (1.0-if64)*(1.0-uu))*
                                (jf64*vv + (1.0-jf64)*(1.0-vv))*
                                (kf64*ww + (1.0-kf64)*(1.0-ww))*
                                c[i][j][k].dot(&weight);
                }
            }
        }
        
        accum
    }
}