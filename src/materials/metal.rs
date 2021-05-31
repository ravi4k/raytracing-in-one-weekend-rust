use crate::geometry::color::Color;
use crate::materials::material::{Material, reflect_ray};
use crate::geometry::vector::{Vector3, Point};
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;
use crate::utils::random_in_unit_sphere;

pub struct Metal {
    pub color: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, in_ray: Ray, intersection: Point, normal: Vector3) -> Ray {
        let reflected_ray = reflect_ray(in_ray.direction, normal);
        Ray {
            origin: intersection,
            direction: reflected_ray + self.fuzz * random_in_unit_sphere(),
            time: in_ray.time,
        }
    }

    fn color(&self) -> Color {
        self.color
    }
}