use crate::textures::texture::Texture;
use std::sync::Arc;
use crate::materials::material::Material;
use crate::geometry::color::Color;
use crate::geometry::vector::Point;
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _in_ray: Ray, _hit_rec: &HitRecord) -> Option<Ray> {
        return Option::None;
    }

    fn color(&self, _u: f32, _v: f32, _intersection: Point) -> Color {
        return Color::BLACK;
    }

    fn emitted(&self, u: f32, v: f32, intersection: Point) -> Color {
        return self.emit.color(u, v, intersection);
    }
}
