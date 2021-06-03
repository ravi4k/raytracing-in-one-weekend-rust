use std::mem::swap;

use crate::geometry::ray::Ray;
use crate::geometry::vector::Point;

#[derive(Clone)]
pub struct AxisAlignedBoundingBox {
    pub minimum: Point,
    pub maximum: Point,
}

impl AxisAlignedBoundingBox {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t0: f32 = t_min;
        let mut t1: f32 = t_max;

        let mut n = ((self.minimum.x - ray.origin.x) / ray.direction.x, (self.maximum.x - ray.origin.x) / ray.direction.x);
        if ray.direction.x < 0.0 {
            swap(&mut n.0, &mut n.1);
        }
        if n.0 > t0 {
            t0 = n.0;
        }
        if n.1 < t1 {
            t1 = n.1;
        }
        if t1 <= t0 {
            return false;
        }

        n = ((self.minimum.y - ray.origin.y) / ray.direction.y, (self.maximum.y - ray.origin.y) / ray.direction.y);
        if ray.direction.y < 0.0 {
            swap(&mut n.0, &mut n.1);
        }
        if n.0 > t0 {
            t0 = n.0;
        }
        if n.1 < t1 {
            t1 = n.1;
        }
        if t1 <= t0 {
            return false;
        }

        n = ((self.minimum.z - ray.origin.z) / ray.direction.z, (self.maximum.z - ray.origin.z) / ray.direction.z);
        if ray.direction.z < 0.0 {
            swap(&mut n.0, &mut n.1);
        }
        if n.0 > t0 {
            t0 = n.0;
        }
        if n.1 < t1 {
            t1 = n.1;
        }
        if t1 <= t0 {
            return false;
        }

        return true;
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        Self {
            minimum: Point {
                x: box0.minimum.x.min(box1.minimum.x),
                y: box0.minimum.y.min(box1.minimum.y),
                z: box0.minimum.z.min(box1.minimum.z),
            },
            maximum: Point {
                x: box0.maximum.x.max(box1.maximum.x),
                y: box0.maximum.y.max(box1.maximum.y),
                z: box0.maximum.z.max(box1.maximum.z),
            },
        }
    }
}
