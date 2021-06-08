use std::cmp::Ordering;
use std::sync::Arc;

use crate::geometry::bounding_volume::AxisAlignedBoundingBox;
use crate::geometry::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Point;

pub trait Hittable: Send + Sync {
    fn hit(&self, _ray: &Ray, _t_min: f32, _t_max: f32) -> Option<f32>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AxisAlignedBoundingBox>;
    fn color(&self, intersection: Point) -> Color;
    fn scatter(&self, in_ray: Ray, intersection: Point) -> Option<Ray>;
    fn emitted(&self, intersection: Point) -> Color;
}

pub struct HitRecord {
    pub object: Arc<dyn Hittable>,
    pub intersection: Point,
}

fn box_compare(_lhs: &Arc<dyn Hittable>, _rhs: &Arc<dyn Hittable>, axis: u32) -> bool {
    let box_a = _lhs.bounding_box(0.0, 0.0);
    let box_b = _rhs.bounding_box(0.0, 0.0);

    match axis {
        0 => box_a.unwrap().minimum.x < box_b.unwrap().minimum.x,
        1 => box_a.unwrap().minimum.y < box_b.unwrap().minimum.y,
        2 | _ =>  box_a.unwrap().minimum.z < box_b.unwrap().minimum.z,
    }
}

pub fn box_cmp_x(_lhs: &Arc<dyn Hittable>, _rhs: &Arc<dyn Hittable>) -> Ordering {
    if box_compare(_rhs, _rhs, 0) {
        return Ordering::Less;
    }
    return Ordering::Greater
}

pub fn box_cmp_y(_lhs: &Arc<dyn Hittable>, _rhs: &Arc<dyn Hittable>) -> Ordering {
    if box_compare(_rhs, _rhs, 1) {
        return Ordering::Less
    }
    return Ordering::Greater
}

pub fn box_cmp_z(_lhs: &Arc<dyn Hittable>, _rhs: &Arc<dyn Hittable>) -> Ordering {
    if box_compare(_rhs, _rhs, 2) {
        return Ordering::Less;
    }
    return Ordering::Greater
}
