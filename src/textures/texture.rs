use crate::geometry::vector::Point;
use crate::geometry::color::Color;

pub trait Texture: Send + Sync {
    fn color(&self, u: f32, v: f32, point: Point) -> Color;
}
