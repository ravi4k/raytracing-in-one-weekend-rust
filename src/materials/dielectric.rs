use crate::geometry::color::Color;
use crate::geometry::vector::{Vector3, Point};
use crate::materials::material::{Material, reflect_ray, reflectance_schlick, refract};
use crate::utils::random_f32;
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Material for Dielectric {
    fn scatter(&self, in_ray: Ray, hit_rec: &HitRecord) -> Option<Ray> {
        let mut ir = 1.0 / self.refractive_index;
        if !hit_rec.front_face {
            ir = self.refractive_index;
        }

        let cos_0 = -in_ray.direction.dot(hit_rec.normal).min(1.0);
        let sin_0 = (1.0 - cos_0.powi(2)).sqrt();

        let is_refracted = ir * sin_0 <= 1.0;
        let mut direction = Vector3::ORIGIN;
        if !is_refracted || reflectance_schlick(cos_0, ir) > random_f32() {
            direction = reflect_ray(in_ray.direction, hit_rec.normal);
        } else {
            direction = refract(in_ray.direction, hit_rec.normal, ir);
        }

        return Option::from(Ray {
            origin: hit_rec.intersection,
            direction,
            time: in_ray.time,
        });
    }

    fn color(&self, _u: f32, _v: f32, _intersection: Point) -> Color {
        return Color::WHITE;
    }
}