use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, Index, IndexMut};

use rand::{Rng, thread_rng};
use rand_distr::StandardNormal;

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ORIGIN: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn direction(self) -> Self {
        (1.0 / self.length()) * self
    }

    pub fn dot(&self, _rhs: Self) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z
    }

    pub fn cross(&self, _rhs: Self) -> Self {
        Self {
            x: self.y * _rhs.z - self.z * _rhs.y,
            y: self.z * _rhs.x - self.x * _rhs.z,
            z: self.x * _rhs.y - self.y * _rhs.x,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        const E: f32= 1e-6;
        self.x.abs() < E && self.y.abs() < E && self.z.abs() < E
    }

    pub fn random_unit_vector() -> Self {
        Self {
            x: thread_rng().sample(StandardNormal),
            y: thread_rng().sample(StandardNormal),
            z: thread_rng().sample(StandardNormal),
        }.direction()
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, _rhs: Self) {
        *self = Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        };
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        Self { x: self.x * _rhs.x, y: self.y * _rhs.y, z: self.z * _rhs.z }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, _rhs: Self) {
        *self = Self {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        };
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Self::Output {
        Self::Output { x: self * _rhs.x, y: self * _rhs.y, z: self * _rhs.z}
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        Self { x: self.x / _rhs.x, y: self.y / _rhs.y, z: self.z / _rhs.z }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f32) -> Self::Output {
        Self::Output { x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs }
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 | _ => &self.z,
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 | _ => &mut self.z,
        }
    }
}

pub type Point = Vector3;
