use std::ops::{Add, AddAssign, Mul, MulAssign};

use image::Rgb;

use crate::utils::random_f32;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn get_pixel(&self, spp: u32) -> image::Rgb<u8> {
        let ir = (255.999 * (self.r / spp as f32).sqrt().clamp(0.0, 1.0)) as u8;
        let ig = (255.999 * (self.g / spp as f32).sqrt().clamp(0.0, 1.0)) as u8;
        let ib = (255.999 * (self.b / spp as f32).sqrt().clamp(0.0, 1.0)) as u8;

        return Rgb([ir, ig, ib]);
    }

    pub fn random() -> Self {
        Self {
            r: random_f32(),
            g: random_f32(),
            b: random_f32(),
        }
    }

    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
}

impl Add for Color {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self { r: self.r + _rhs.r, g: self.g + _rhs.g, b: self.b + _rhs.b }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, _rhs: Self) {
        *self = Self {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b,
        };
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        Self { r: self.r * _rhs.r, g: self.g * _rhs.g, b: self.b * _rhs.b }
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, _rhs: Self) {
        *self = Self {
            r: self.r * _rhs.r,
            g: self.g * _rhs.g,
            b: self.b * _rhs.b,
        };
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Self::Output {
        Self::Output { r: self * _rhs.r, g: self * _rhs.g, b: self * _rhs.b }
    }
}
