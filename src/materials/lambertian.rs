use crate::geometry::color::Color;
use crate::geometry::vector::Vector3;
use crate::materials::material::Material;

pub struct Lambertian {
    pub color: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _in_direction: Vector3, normal: Vector3) -> Vector3 {
        let mut scatter_direction = normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        scatter_direction.direction()
    }

    fn color(&self) -> Color {
        self.color
    }
}