use crate::geometry::color::Color;
use crate::materials::material::{Material, reflect_ray};
use crate::geometry::vector::Vector3;
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;

pub struct Metal {
    pub color: Color,
}

impl Material for Metal {
    fn scatter(&self, in_direction: Vector3, hit_rec: HitRecord) -> Ray {
        let normal = hit_rec.object.unwrap().normal(hit_rec.intersection);
        let reflected_ray = reflect_ray(in_direction, normal);
        Ray {
            origin: hit_rec.intersection,
            direction: reflected_ray,
        }
    }

    fn color(&self) -> Color {
        self.color
    }
}