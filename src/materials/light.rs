use std::sync::Arc;

use crate::geometry::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Point;
use crate::materials::material::Material;
use crate::objects::hittable::HitRecord;
use crate::textures::solid::SolidColor;
use crate::textures::texture::Texture;

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(color: Color) -> Self {
        return Self {
            emit: Arc::new(SolidColor { color })
        };
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _ray: Ray, hit_rec: &HitRecord, u: f32, v: f32, intersection: Point) -> Color {
        return if hit_rec.front_face {
            self.emit.color(u, v, intersection)
        } else {
            Color::BLACK
        };
    }
}
