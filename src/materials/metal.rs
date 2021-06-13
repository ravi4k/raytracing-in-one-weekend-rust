use crate::geometry::color::Color;
use crate::geometry::vector::Point;
use crate::materials::material::{Material, reflect_ray};
use crate::utils::random_in_unit_sphere;
use crate::objects::hittable::HitRecord;
use crate::geometry::ray::Ray;

pub struct Metal {
    pub color: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, in_ray: Ray, hit_rec: &HitRecord) -> Option<Ray> {
        let reflected_direction = reflect_ray(in_ray.direction, hit_rec.normal) + self.fuzz * random_in_unit_sphere();
        if reflected_direction.dot(hit_rec.normal) < 0.0 {
            return Option::None;
        }
        return Option::from(Ray {
            origin: hit_rec.intersection,
            direction: reflected_direction.direction(),
            time: in_ray.time,
        });
    }

    fn color(&self, _u: f32, _v: f32, _intersection: Point) -> Color {
        self.color
    }
}