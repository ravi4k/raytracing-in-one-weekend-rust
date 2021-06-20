use std::sync::Arc;

use crate::geometry::bounding_volume::AxisAlignedBoundingBox;
use crate::geometry::ray::Ray;
use crate::geometry::vector::{Point, Vector3};
use crate::materials::material::Material;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::utils::{INF_F32, random_f32_range};

pub struct XYRect {
    pub x: (f32, f32),
    pub y: (f32, f32),
    pub k: f32,
    pub material: Arc<dyn Material>,
}

impl XYRect {
    const NORMAL: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };

    fn get_uv(&self, x: f32, y: f32) -> (f32, f32) {
        return ((x - self.x.0) / (self.x.1 - self.x.0),
                (y - self.y.0) / (self.y.1 - self.y.0));
    }
}

impl Hittable for XYRect {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return Option::None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return Option::None;
        }

        let intersection = ray.at_distance(t);
        let normal = Self::NORMAL;
        let material = self.material.clone();
        let (u, v) = self.get_uv(x, y);

        let mut hit_rec = HitRecord { intersection, normal, material, t, u, v, front_face: false };
        hit_rec.set_face_normal(ray);
        return Option::from(hit_rec);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AxisAlignedBoundingBox> {
        return Option::from(AxisAlignedBoundingBox {
            minimum: Point {
                x: self.x.0,
                y: self.y.0,
                z: self.k - 0.0001,
            },
            maximum: Point {
                x: self.x.1,
                y: self.y.1,
                z: self.k + 0.0001,
            },
        });
    }
}

pub struct XZRect {
    pub x: (f32, f32),
    pub z: (f32, f32),
    pub k: f32,
    pub material: Arc<dyn Material>,
}

impl XZRect {
    const NORMAL: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };

    fn get_uv(&self, x: f32, z: f32) -> (f32, f32) {
        return ((x - self.x.0) / (self.x.1 - self.x.0),
                (z - self.z.0) / (self.z.1 - self.z.0));
    }
}

impl Hittable for XZRect {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return Option::None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if x < self.x.0 || x > self.x.1 || z < self.z.0 || z > self.z.1 {
            return Option::None;
        }

        let intersection = ray.at_distance(t);
        let normal = Self::NORMAL;
        let material = self.material.clone();
        let (u, v) = self.get_uv(x, z);

        let mut hit_rec = HitRecord { intersection, normal, material, t, u, v, front_face: false };
        hit_rec.set_face_normal(ray);
        return Option::from(hit_rec);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AxisAlignedBoundingBox> {
        return Option::from(AxisAlignedBoundingBox {
            minimum: Point {
                x: self.x.0,
                y: self.k - 0.0001,
                z: self.z.0,
            },
            maximum: Point {
                x: self.x.1,
                y: self.k + 0.0001,
                z: self.z.1,
            },
        });
    }

    fn pdf_value(&self, origin: Point, v: Vector3) -> f32 {
        let opt_hit_rec = self.hit(Ray { origin, direction: v, time: 0.0 }, 0.001, INF_F32);
        if opt_hit_rec.is_none() {
            return 0.0;
        }

        let hit_rec = opt_hit_rec.unwrap();
        let area = (self.x.1 - self.x.0) * (self.z.1 - self.z.0);
        let dist_squared = hit_rec.t.powi(2) * v.length_squared();
        let cosine = (v.dot(hit_rec.normal) / v.length()).abs();

        return dist_squared / (cosine * area);
    }

    fn random(&self, origin: Vector3) -> Vector3 {
        let random_point = Point {
            x: random_f32_range(self.x.0, self.x.1),
            y: self.k,
            z: random_f32_range(self.z.0, self.z.1),
        };
        return random_point - origin;
    }
}

pub struct YZRect {
    pub y: (f32, f32),
    pub z: (f32, f32),
    pub k: f32,
    pub material: Arc<dyn Material>,
}

impl YZRect {
    const NORMAL: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };

    fn get_uv(&self, y: f32, z: f32) -> (f32, f32) {
        return ((y - self.y.0) / (self.y.1 - self.y.0),
                (z - self.z.0) / (self.z.1 - self.z.0));
    }
}

impl Hittable for YZRect {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return Option::None;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;

        if y < self.y.0 || y > self.y.1 || z < self.z.0 || z > self.z.1 {
            return Option::None;
        }

        let intersection = ray.at_distance(t);
        let normal = Self::NORMAL;
        let material = self.material.clone();
        let (u, v) = self.get_uv(y, z);

        let mut hit_rec = HitRecord { intersection, normal, material, t, u, v, front_face: false };
        hit_rec.set_face_normal(ray);
        return Option::from(hit_rec);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AxisAlignedBoundingBox> {
        return Option::from(AxisAlignedBoundingBox {
            minimum: Point {
                x: self.k - 0.0001,
                y: self.y.0,
                z: self.z.0,
            },
            maximum: Point {
                x: self.k + 0.0001,
                y: self.y.1,
                z: self.z.1,
            },
        });
    }
}
