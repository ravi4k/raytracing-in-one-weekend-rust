use crate::geometry::bounding_volume::AxisAlignedBoundingBox;
use crate::geometry::ray::Ray;
use crate::geometry::vector::{Point, Vector3};
use crate::materials::material::Material;
use crate::objects::hittable::{Hittable, HitRecord};
use crate::utils::PI;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    fn normal(&self, intersection: Point) -> Vector3 {
        let normal = (intersection - self.center).direction();
        if self.radius < 0.0 {
            return -normal;
        }
        return normal;
    }

    fn get_sphere_uv(point: Point) -> (f32, f32) {
        let theta = (-(point.y)).acos();
        let phi = (-(point.z)).atan2(point.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        return (u, v);
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let mut discriminant = b.powi(2) - c;
        if discriminant < 0.0 {
            return Option::None;
        }
        discriminant = discriminant.sqrt();

        let mut t = -b - discriminant;
        if t < t_min || t > t_max {
            t = -b + discriminant;
            if t < t_min || t > t_max {
                return Option::None;
            }
        }

        let intersection = ray.at_distance(t);
        let normal = self.normal(intersection);
        let material = self.material.clone();
        let (u, v) = Self::get_sphere_uv(normal);

        let mut hit_rec = HitRecord { intersection, normal, material, t, u, v, front_face: false };
        hit_rec.set_face_normal(ray);
        return Option::from(hit_rec);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AxisAlignedBoundingBox> {
        let p = Point {x: self.radius, y: self.radius, z: self.radius};
        return Option::from(AxisAlignedBoundingBox {
            minimum: self.center - p,
            maximum: self.center + p,
        });
    }
}

pub struct MovingSphere {
    pub centre0: Point,
    pub center1: Point,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Arc<dyn Material>,
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
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let mut discriminant = b.powi(2) - c;
        if discriminant < 0.0 {
            return Option::None;
        }
        discriminant = discriminant.sqrt();

        let mut t = -b - discriminant;
        if t < t_min || t > t_max {
            t = -b + discriminant;
            if t < t_min || t > t_max {
                return Option::None;
            }
        }

        let intersection = ray.at_distance(t);
        let normal = self.normal(intersection, ray.time);
        let material = self.material.clone();
        let (u, v) = Sphere::get_sphere_uv(normal);

        let mut hit_rec = HitRecord { intersection, normal, material, t, u, v, front_face: false };
        hit_rec.set_face_normal(ray);
        return Option::from(hit_rec);
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AxisAlignedBoundingBox> {
        let p = Point {x: self.radius, y: self.radius, z: self.radius};
        let box0 = AxisAlignedBoundingBox {
            minimum: self.center(t0) - p,
            maximum: self.center(t0) + p,
        };
        let box1 = AxisAlignedBoundingBox {
            minimum: self.center(t1) - p,
            maximum: self.center(t1) + p,
        };
        Option::from(AxisAlignedBoundingBox::surrounding_box(box0, box1))
    }
}
