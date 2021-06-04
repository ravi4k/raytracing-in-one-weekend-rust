use crate::geometry::color::Color;
use crate::geometry::vector::{Vector3, Point};
use crate::materials::material::{Material, reflect_ray};
use crate::utils::random_in_unit_sphere;

pub struct Metal {
    pub color: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, in_direction: Vector3, normal: Vector3) -> Vector3 {
        let reflected_ray = reflect_ray(in_direction, normal);
        reflected_ray + self.fuzz * random_in_unit_sphere()
    }

    fn color(&self, _u: f32, _v: f32, _intersection: Point) -> Color {
        self.color
    }
}