use std::sync::Arc;

use crate::geometry::bounding_volume::AxisAlignedBoundingBox;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Point;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::utils::{degrees_to_radians, INF_F32};

pub struct Translate {
    pub object: Arc<dyn Hittable>,
    pub offset: Point,
}

impl Translate {
    fn translated_ray(&self, in_ray: Ray) -> Ray {
        return Ray {
            origin: in_ray.origin - self.offset,
            direction: in_ray.direction,
            time: in_ray.time,
        };
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let translated_ray = self.translated_ray(ray);
        let opt_hit_rec = self.object.hit(translated_ray, t_min, t_max);
        if opt_hit_rec.is_none() {
            return Option::None;
        }

        let mut hit_rec = opt_hit_rec.unwrap();
        hit_rec.intersection += self.offset;
        hit_rec.set_face_normal(translated_ray);
        return Option::from(hit_rec);
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AxisAlignedBoundingBox> {
        let opt_bound_box = self.object.bounding_box(t0, t1);
        if opt_bound_box.is_none() {
            return Option::None;
        }
        let bound_box = opt_bound_box.unwrap();
        return Option::from(AxisAlignedBoundingBox {
            minimum: bound_box.minimum + self.offset,
            maximum: bound_box.maximum + self.offset,
        });
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bound_box: Option<AxisAlignedBoundingBox>,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f32) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut temp = Self {
            object,
            sin_theta,
            cos_theta,
            bound_box: Option::None,
        };

        let opt_bound_box = temp.object.bounding_box(0.0, 1.0);
        if opt_bound_box.is_none() {
            return temp;
        }

        let mut bound_box = opt_bound_box.unwrap();
        let mut min = Point { x: INF_F32, y: INF_F32, z: INF_F32 };
        let mut max = Point { x: -INF_F32, y: -INF_F32, z: -INF_F32 };

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bound_box.maximum.x + (1 - i) as f32 * bound_box.minimum.x;
                    let y = j as f32 * bound_box.maximum.y + (1 - j) as f32 * bound_box.minimum.y;
                    let z = k as f32 * bound_box.maximum.z + (1 - k) as f32 * bound_box.minimum.z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Point { x: new_x, y, z: new_z };
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bound_box = AxisAlignedBoundingBox { minimum: min, maximum: max };
        temp.bound_box = Option::from(bound_box);
        return temp;
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray { origin, direction, time: ray.time };
        let opt_hit_rec = self.object.hit(rotated_ray, t_min, t_max);
        if opt_hit_rec.is_none() {
            return Option::None;
        }

        let mut hit_rec = opt_hit_rec.unwrap();
        let mut intersection = hit_rec.intersection;
        let mut normal = hit_rec.normal;

        intersection[0] = self.cos_theta * hit_rec.intersection[0] + self.sin_theta * hit_rec.intersection[2];
        intersection[2] = -self.sin_theta * hit_rec.intersection[0] + self.cos_theta * hit_rec.intersection[2];

        normal[0] = self.cos_theta * hit_rec.normal[0] + self.sin_theta * hit_rec.normal[2];
        normal[2] = -self.sin_theta * hit_rec.normal[0] + self.cos_theta * hit_rec.normal[2];

        hit_rec.intersection = intersection;
        hit_rec.normal = normal;
        hit_rec.set_face_normal(rotated_ray);

        return Option::from(hit_rec);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AxisAlignedBoundingBox> {
        return self.bound_box.clone();
    }
}
