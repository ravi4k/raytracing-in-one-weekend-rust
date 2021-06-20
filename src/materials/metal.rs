use crate::geometry::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Point;
use crate::materials::material::{Material, reflect_ray, ScatterRecord};
use crate::objects::hittable::HitRecord;
use crate::utils::random_in_unit_sphere;

pub struct Metal {
    pub color: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, in_ray: Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected_direction = reflect_ray(in_ray.direction, hit_rec.normal) + self.fuzz * random_in_unit_sphere();
        return Option::from(ScatterRecord {
            specular_ray: Option::from(Ray { origin: hit_rec.intersection, direction: reflected_direction, time: in_ray.time }),
            attenuation: self.color,
            pdf_ptr: None,
        });
    }
}