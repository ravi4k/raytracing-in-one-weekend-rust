use crate::textures::texture::Texture;
use std::sync::Arc;
use crate::materials::material::Material;
use crate::geometry::color::Color;
use crate::geometry::vector::{Point, Vector3};
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;

pub struct Isotropic {
    pub color: Arc<dyn Texture>,
}

impl Material for Isotropic {
    fn scatter(&self, in_ray: Ray, hit_rec: &HitRecord) -> Option<Ray> {
        return Option::from(Ray {
            origin: hit_rec.intersection,
            direction: Vector3::random_unit_vector(),
            time: in_ray.time,
        });
    }

    fn color(&self, u: f32, v: f32, intersection: Point) -> Color {
        return self.color.color(u, v, intersection);
    }
}
