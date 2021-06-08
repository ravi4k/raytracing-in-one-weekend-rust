use std::sync::Arc;
use crate::materials::material::Material;
use crate::objects::hittable::Hittable;
use crate::geometry::bounding_volume::AxisAlignedBoundingBox;
use crate::geometry::color::Color;
use crate::geometry::vector::{Point, Vector3};
use crate::geometry::ray::Ray;

pub struct XYRect {
    pub x: (f32, f32),
    pub y: (f32, f32),
    pub k: f32,
    pub material: Arc<dyn Material>
}

impl XYRect {
    fn normal() -> Vector3 {
        return Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    }

    fn get_uv(&self, x: f32, y: f32) -> (f32, f32) {
        return ((x - self.x.0) / (self.x.1 - self.x.0),
                (y - self.y.0) / (self.y.1 - self.y.0));
    }
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<f32> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return Option::None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return Option::None;
        }

        return Option::from(t);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AxisAlignedBoundingBox> {
        Option::from(AxisAlignedBoundingBox {
            minimum: Point {
                x: self.x.0,
                y: self.y.0,
                z: self.k - 0.0001,
            },
            maximum: Point {
                x: self.x.1,
                y: self.y.1,
                z: self.k + 0.0001,
            }
        })
    }

    fn color(&self, intersection: Point) -> Color {
        let (u, v) = self.get_uv(intersection.x, intersection.y);
        return self.material.color(u, v, intersection);
    }

    fn scatter(&self, in_ray: Ray, intersection: Point) -> Option<Ray> {
        let scattered_direction = self.material.scatter(in_ray.direction, Self::normal());

        if scattered_direction.is_none() {
            return Option::None;
        }

        return Option::from(Ray {
            origin: intersection,
            direction: scattered_direction.unwrap(),
            time: in_ray.time,
        });
    }

    fn emitted(&self, intersection: Point) -> Color {
        let (u, v) = self.get_uv(intersection.x, intersection.y);
        return self.material.emitted(u, v, intersection);
    }
}
