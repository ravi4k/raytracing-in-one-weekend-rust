use std::ops::{Add, Mul, AddAssign, MulAssign};
use image::Rgb;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

impl Color {
    pub fn get_pixel(&self, spp: u32) -> image::Rgb<u8> {
        let ir = (255.999 * clamp(self.r / spp as f32, 0.0, 1.0).sqrt()) as u8;
        let ig = (255.999 * clamp(self.g / spp as f32, 0.0, 1.0).sqrt()) as u8;
        let ib = (255.999 * clamp(self.b / spp as f32, 0.0, 1.0).sqrt()) as u8;

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
