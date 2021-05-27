use std::ops::{Add, Mul, AddAssign, MulAssign};
use image::Rgb;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn get_pixel(self) -> image::Rgb<u8> {
        let ir = (255.999 * self.r) as u8;
        let ig = (255.999 * self.g) as u8;
        let ib = (255.999 * self.b) as u8;

        Rgb([ir, ig, ib])
    }
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
        Self::Output { r: self * _rhs.r, g: self * _rhs.g, b: self * _rhs.b}
    }
}
