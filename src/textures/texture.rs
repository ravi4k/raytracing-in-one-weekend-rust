use crate::geometry::color::Color;
use crate::geometry::vector::Point;

pub trait Texture: Send + Sync {
    fn color(&self, u: f32, v: f32, point: Point) -> Color;
}
