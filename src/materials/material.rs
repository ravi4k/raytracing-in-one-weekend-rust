use crate::objects::hittable::HitRecord;
use crate::geometry::ray::Ray;
use crate::geometry::vector::{Vector3, Point};
use crate::geometry::color::Color;

pub trait Material: Send + Sync {
    fn scatter(&self, in_ray: Ray, intersection: Point, normal: Vector3) -> Ray;
    fn color(&self) -> Color;
}

pub fn is_front_face(in_direction: Vector3, normal: Vector3) -> bool {
    in_direction.dot(normal) < 0.0
}

pub fn reflect_ray(in_direction: Vector3, normal: Vector3) -> Vector3 {
    (in_direction - 2.0 * in_direction.dot(normal) * normal).direction()
}

pub fn refract(in_direction: Vector3, normal: Vector3, ir: f32) -> Vector3 {
    let cos_0 = -in_direction.dot(normal).min(1.0);
    let refract_perp = ir *  (in_direction + cos_0 * normal);
    let refract_para = -(1.0 - refract_perp.length_squared()).sqrt() * normal;
    (refract_perp + refract_para).direction()
}

pub fn reflectance_schlick(cos_0: f32, ir: f32) -> f32 {
    let mut r0 = (1.0 - ir) / (1.0 + ir);
    r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_0).powi(5)
}