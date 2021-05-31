use crate::materials::material::{Material, refract, reflect_ray, reflectance_schlick, is_front_face};
use crate::geometry::color::Color;
use crate::geometry::vector::{Vector3, Point};
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;
use crate::utils::random_f32;

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Material for Dielectric {
    fn scatter(&self, in_ray: Ray, intersection: Point, _normal: Vector3) -> Ray {
        let mut normal = _normal;
        let front_face = is_front_face(in_ray.direction, normal);
        if !front_face {
            normal = -normal;
        }

        let mut ir = 1.0 / self.refractive_index;
        if !front_face {
            ir = self.refractive_index;
        }

        let cos_0 = -in_ray.direction.dot(normal).min(1.0);
        let sin_0 = (1.0 - cos_0.powi(2)).sqrt();
        if ir * sin_0 > 1.0 || reflectance_schlick(cos_0, ir) > random_f32() {
            return Ray {
                origin: intersection,
                direction: reflect_ray(in_ray.direction, normal),
                time: in_ray.time,
            }
        }

        Ray {
            origin: intersection,
            direction: refract(in_ray.direction, normal, ir),
            time: in_ray.time,
        }
    }

    fn color(&self) -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0 }
    }
}