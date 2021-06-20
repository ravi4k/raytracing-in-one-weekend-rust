use crate::geometry::color::Color;
use crate::geometry::vector::Point;
use crate::textures::texture::Texture;

pub struct SolidColor {
    pub color: Color,
}

impl Texture for SolidColor {
    fn color(&self, _u: f32, _v: f32, _point: Point) -> Color {
        return self.color;
    }
}