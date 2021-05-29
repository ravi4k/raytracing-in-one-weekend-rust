use crate::materials::material::{Material, refract, reflect_ray, reflectance_schlick, is_front_face};
use crate::geometry::color::Color;
use crate::geometry::vector::Vector3;
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;
use crate::utils::random_f32;

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Material for Dielectric {
    fn scatter(&self, in_direction: Vector3, hit_rec: HitRecord) -> Ray {
        let mut normal = hit_rec.object.unwrap().normal(hit_rec.intersection);
        let front_face = is_front_face(in_direction, normal);
        if !front_face {
            normal = -normal;
        }

        let mut ir = 1.0 / self.refractive_index;
        if !front_face {
            ir = self.refractive_index;
        }

        let cos_0 = -in_direction.dot(normal).min(1.0);
        let sin_0 = (1.0 - cos_0.powi(2)).sqrt();
        if ir * sin_0 > 1.0 || reflectance_schlick(cos_0, ir) > random_f32() {
            return Ray {
                origin: hit_rec.intersection,
                direction: reflect_ray(in_direction, normal),
            }
        }

        Ray {
            origin: hit_rec.intersection,
            direction: refract(in_direction, normal, ir),
        }
    }

    fn color(&self) -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0 }
    }
}