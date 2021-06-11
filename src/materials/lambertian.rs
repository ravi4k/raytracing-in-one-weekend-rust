use crate::geometry::color::Color;
use crate::geometry::vector::{Vector3, Point};
use crate::materials::material::Material;
use crate::textures::texture::Texture;
use std::sync::Arc;
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, in_ray: Ray, hit_rec: &HitRecord) -> Option<Ray> {
        let mut scatter_direction = hit_rec.normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }
        return Option::from(Ray {
            origin: hit_rec.intersection,
            direction: scatter_direction.direction(),
            time: in_ray.time,
        });
    }

    fn color(&self, u: f32, v: f32, intersection: Point) -> Color {
        self.albedo.color(u, v, intersection)
    }
}
