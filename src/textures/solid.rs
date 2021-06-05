use crate::geometry::color::Color;
use crate::textures::texture::Texture;
use crate::geometry::vector::Point;

pub struct SolidColor {
    pub color: Color,
}

impl Texture for SolidColor {
    fn color(&self, _u: f32, _v: f32, _point: Point) -> Color {
        return self.color;
    }
}