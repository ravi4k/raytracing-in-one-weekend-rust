use crate::geometry::color::Color;
use crate::geometry::vector::Vector3;
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;
use crate::materials::material::Material;

pub struct Lambertian {
    pub color: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _in_direction: Vector3, hit_rec: HitRecord) -> Ray {
        let normal = hit_rec.object.unwrap().normal(hit_rec.intersection);
        let mut scatter_direction = normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        Ray {
            origin: hit_rec.intersection,
            direction: scatter_direction.direction(),
        }
    }

    fn color(&self) -> Color {
        self.color
    }
}