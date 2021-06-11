use std::sync::Arc;

use crate::geometry::bounding_volume::AxisAlignedBoundingBox;
use crate::geometry::ray::Ray;
use crate::objects::hittable::{box_cmp_x, box_cmp_y, box_cmp_z, Hittable, HitRecord};
use crate::utils::random_int;

pub struct BVHNode {
    pub bound_box: AxisAlignedBoundingBox,
    pub left_node: Arc<dyn Hittable>,
    pub right_node: Arc<dyn Hittable>,
}

impl BVHNode {
    pub fn create_tree(objects: &mut [Arc<dyn Hittable>], time0: f32, time1: f32) -> Arc<dyn Hittable> {
        let axis = random_int(0, 2);
        let comparator = match axis {
            0 => box_cmp_x,
            1 => box_cmp_y,
            2 | _ => box_cmp_z,
        };

        if objects.len() == 1 {
            return objects[0].clone();
        }

        objects.sort_unstable_by(|a, b| comparator(a, b));
        let mid_idx = objects.len() / 2;
        let left_node = Self::create_tree(&mut objects[..mid_idx], time0, time1);
        let right_node = Self::create_tree(&mut objects[mid_idx..], time0, time1);

        let box_left = left_node.bounding_box(time0, time1).unwrap();
        let box_right = right_node.bounding_box(time0, time1).unwrap();

        Arc::new(BVHNode {
            bound_box: AxisAlignedBoundingBox::surrounding_box(box_left, box_right),
            left_node,
            right_node,
        })
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bound_box.hit(ray, t_min, t_max) {
            return Option::None
        }

        let mut _t_max = t_max;
        let hit_left = self.left_node.hit(ray, t_min, _t_max);
        if hit_left.is_some() {
            _t_max = (ray.origin - hit_left.as_ref().unwrap().intersection).length();
        }

        let hit_right = self.right_node.hit(ray, t_min, _t_max);
        if hit_right.is_some() {
            return hit_right;
        }
        return hit_left;
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AxisAlignedBoundingBox> {
        return Option::from(self.bound_box.clone());
    }
}
