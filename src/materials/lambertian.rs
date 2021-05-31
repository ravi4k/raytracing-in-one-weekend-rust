use crate::geometry::color::Color;
use crate::geometry::vector::{Vector3, Point};
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;
use crate::materials::material::Material;

pub struct Lambertian {
    pub color: Color,
}

impl Material for Lambertian {
    fn scatter(&self, in_ray: Ray, intersection: Point, normal: Vector3) -> Ray {
        let mut scatter_direction = normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        Ray {
            origin: intersection,
            direction: scatter_direction.direction(),
            time: in_ray.time,
        }
    }

    fn color(&self) -> Color {
        self.color
    }
}