use std::mem::swap;

use crate::geometry::ray::Ray;
use crate::geometry::vector::Point;

#[derive(Clone)]
pub struct AxisAlignedBoundingBox {
    pub minimum: Point,
    pub maximum: Point,
}

impl AxisAlignedBoundingBox {
    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in 0..3 {
            let mut t0 = (self.minimum[a] - ray.origin[a]) / ray.direction[a];
            let mut t1 = (self.maximum[a] - ray.origin[a]) / ray.direction[a];

            if ray.direction[a] < 0.0 {
                swap(&mut t0, &mut t1);
            }

            if t0 > t_min { t_min = t0; }
            if t1 < t_max { t_max = t1; }
            if t_max <= t_min { return false; }
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
