use crate::objects::hittable::HitRecord;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector3;
use crate::geometry::color::Color;

pub trait Material {
    fn scatter(&self, in_direction: Vector3, hit_rec: HitRecord) -> Ray;
    fn color(&self) -> Color;
}

pub fn reflect_ray(in_direction: Vector3, normal: Vector3) -> Vector3 {
    (in_direction - 2.0 * in_direction.dot(normal) * normal).direction()
}