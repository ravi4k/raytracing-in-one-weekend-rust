use crate::geometry::vector::{Point, Vector3};
use crate::geometry::ray::Ray;
use crate::objects::hittable::{Hittable, HitRecord};
use crate::materials::material::Material;
use crate::geometry::color::Color;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    fn normal(&self, _intersection: Point) -> Vector3 {
        let normal = (_intersection - self.center).direction();
        if self.radius < 0.0 {
            return -normal;
        }
        return normal;
    }
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

    fn color(&self) -> Color {
        self.material.color()
    }

    fn scatter(&self, in_ray: Ray, hit_rec: HitRecord) -> Ray {
        self.material.scatter(in_ray, hit_rec.intersection, self.normal(hit_rec.intersection))
    }
}

pub(crate) struct MovingSphere {
    pub centre0: Point,
    pub center1: Point,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl MovingSphere {
    fn center(&self, time: f32) -> Point {
        self.centre0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.centre0)
    }

    fn normal(&self, intersection: Point, time: f32) -> Vector3 {
        let normal = (intersection - self.center(time)).direction();
        if self.radius < 0.0 {
            return -normal;
        }
        return normal;
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> f32 {
        let oc = ray.origin - self.center(ray.time);
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

    fn color(&self) -> Color {
        self.material.color()
    }

    fn scatter(&self, in_ray: Ray, hit_rec: HitRecord) -> Ray {
        let normal = self.normal(hit_rec.intersection, in_ray.time);
        self.material.scatter(in_ray, hit_rec.intersection, normal)
    }
}
