use crate::geometry::color::Color;
use crate::geometry::vector::{Vector3, Point};
use crate::materials::material::{is_front_face, Material, reflect_ray, reflectance_schlick, refract};
use crate::utils::random_f32;

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Material for Dielectric {
    fn scatter(&self, in_direction: Vector3, _normal: Vector3) -> Option<Vector3> {
        let mut normal = _normal;
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
            return Option::from(reflect_ray(in_direction, normal));
        }

        return Option::from(refract(in_direction, normal, ir));
    }

    fn color(&self, _u: f32, _v: f32, _intersection: Point) -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0 }
    }
}