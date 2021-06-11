use std::cmp::Ordering;
use std::sync::Arc;

use crate::geometry::bounding_volume::AxisAlignedBoundingBox;
use crate::geometry::ray::Ray;
use crate::geometry::vector::{Vector3, Point};
use crate::materials::material::Material;

pub struct HitRecord {
    pub intersection: Point,
    pub normal: Vector3,
    pub material: Arc<dyn Material>,
    pub t: f32,
    pub u: f32, pub v: f32,
    pub front_face: bool,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AxisAlignedBoundingBox>;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray) {
        self.front_face = ray.direction.dot(self.normal) < 0.0;
        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}

fn box_compare(lhs: &Arc<dyn Hittable>, rhs: &Arc<dyn Hittable>, axis: usize) -> bool {
    let box_a = lhs.bounding_box(0.0, 0.0).unwrap();
    let box_b = rhs.bounding_box(0.0, 0.0).unwrap();

    return box_a.minimum[axis] < box_b.minimum[axis];
}

pub fn box_cmp_x(lhs: &Arc<dyn Hittable>, rhs: &Arc<dyn Hittable>) -> Ordering {
    if box_compare(lhs, rhs, 0) {
        return Ordering::Less;
    }
    return Ordering::Greater;
}

pub fn box_cmp_y(lhs: &Arc<dyn Hittable>, rhs: &Arc<dyn Hittable>) -> Ordering {
    if box_compare(lhs, rhs, 1) {
        return Ordering::Less
    }
    return Ordering::Greater;
}

pub fn box_cmp_z(lhs: &Arc<dyn Hittable>, rhs: &Arc<dyn Hittable>) -> Ordering {
    if box_compare(lhs, rhs, 2) {
        return Ordering::Less;
    }
    return Ordering::Greater;
}
