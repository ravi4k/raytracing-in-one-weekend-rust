use std::ops::{Add, Mul, AddAssign, MulAssign};

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
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