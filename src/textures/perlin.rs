use crate::textures::texture::Texture;
use crate::geometry::color::Color;
use crate::geometry::vector::{Point, Vector3};
use crate::utils::random_int;

pub struct Perlin {
    ran_vec: Vec<Vector3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut ran_vec: Vec<Vector3> = Vec::with_capacity(Self::POINT_COUNT);
        for _i in 0..Self::POINT_COUNT {
            ran_vec.push(Vector3::random_unit_vector() );
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        return Self {
            ran_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: Point) -> f32 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let def = Vector3 { x: 0.0, y: 0.0, z: 0.0};
        let def_z = vec![def, def];
        let def_y = vec![def_z.clone(), def_z];
        let mut def_x = vec![def_y.clone(), def_y];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    def_x[di][dj][dk] = self.ran_vec[
                        (self.perm_x[((i + di as i32) & 255) as usize] ^
                         self.perm_y[((j + dj as i32) & 255) as usize] ^
                         self.perm_z[((k + dk as i32) & 255) as usize] as usize)
                    ];
                }
            }
        }

        return Self::perlin_interp(def_x, u, v, w);
    }

    fn perlin_interp(c: Vec<Vec<Vec<Vector3>>>, u: f32, v: f32, w: f32) -> f32 {
        let uu = u.powi(2) * (3.0 - 2.0 * u);
        let vv = v.powi(2) * (3.0 - 2.0 * v);
        let ww = w.powi(2) * (3.0 - 2.0 * w);


        let mut accum: f32 = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vector3 {
                        x: u - i as f32,
                        y: v - j as f32,
                        z: w - k as f32,
                    };
                    accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu)) *
                             (j as f32 * vv + (1 - j) as f32 * (1.0 - vv)) *
                             (k as f32 * ww + (1 - k) as f32 * (1.0 - ww)) *
                             c[i][j][k].dot(weight);
                }
            }
        }

        return accum;
    }

    pub fn turb(&self, point: Point, depth: u32) -> f32 {
        let mut accum: f32 = 0.0;
        let mut temp_p = point;
        let mut weight: f32 = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        }

        return accum.abs();
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p: Vec<usize> = Vec::with_capacity(Self::POINT_COUNT);
        for i in 0..Self::POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p);
        return p;
    }

    fn permute(points: &mut Vec<usize>) {
        for i in (0..points.len()).rev() {
            let target = random_int(0, i as u32);
            points.swap(i, target as usize);
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f32,
}

impl Texture for NoiseTexture {
    fn color(&self, _u: f32, _v: f32, point: Point) -> Color {
        return 0.5 * (1.0 + (self.scale * point.z + 10.0 * self.noise.turb(point, 7)).sin()) * Color { r: 1.0, g: 1.0, b: 1.0 };
    }
}
