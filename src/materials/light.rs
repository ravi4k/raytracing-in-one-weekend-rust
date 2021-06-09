use crate::textures::texture::Texture;
use std::sync::Arc;
use crate::materials::material::Material;
use crate::geometry::color::Color;
use crate::geometry::vector::{Vector3, Point};

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _in_direction: Vector3, _normal: Vector3, _front_face: bool) -> Option<Vector3> {
        return Option::None;
    }

    fn color(&self, u: f32, v: f32, intersection: Point) -> Color {
        return self.emit.color(u, v, intersection);
    }

    fn emitted(&self, u: f32, v: f32, intersection: Point) -> Color {
        return self.emit.color(u, v, intersection);
    }
}
