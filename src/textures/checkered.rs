use crate::textures::texture::Texture;
use std::sync::Arc;
use crate::geometry::color::Color;
use crate::geometry::vector::Point;

pub struct CheckeredTexture {
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}

impl Texture for CheckeredTexture {
    fn color(&self, u: f32, v: f32, point: Point) -> Color {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        return if sines < 0.0 {
            self.odd.color(u, v, point)
        } else {
            self.even.color(u, v, point)
        }
    }
}
