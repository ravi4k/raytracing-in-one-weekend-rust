use crate::geometry::vector::{Point, Vector3};
use crate::geometry::ray::Ray;
use crate::objects::hittable::{Hittable, HitRecord};
use crate::materials::material::Material;
use crate::geometry::color::Color;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub(crate) material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> f32 {
        let oc = ray.origin - self.center;
        let b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let mut discriminant = b.powi(2) - c;
        if discriminant < 0.0 {
            return -1.0;
        }
        discriminant = discriminant.sqrt();

        let mut t = -b - discriminant;
        if t < t_min || t > t_max {
            t = -b + discriminant;
            if t < t_min || t > t_max {
                return -1.0;
            }
        }
        return t;
    }

    fn normal(&self, _intersection: Point) -> Vector3 {
        let normal = (_intersection - self.center).direction();
        if self.radius < 0.0 {
            return -normal;
        }
        return normal;
    }

    fn color(&self) -> Color {
        self.material.color()
    }

    fn scatter(&self, in_direction: Vector3, hit_rec: HitRecord) -> Ray {
        self.material.scatter(in_direction, hit_rec)
    }
}