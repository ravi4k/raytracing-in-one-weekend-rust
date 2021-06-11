use crate::geometry::vector::Point;
use crate::objects::hittable::{Hittable, HitRecord};
use std::sync::Arc;
use crate::world::hittable_list::HittableList;
use crate::geometry::bounding_volume::AxisAlignedBoundingBox;
use crate::geometry::ray::Ray;
use crate::materials::material::Material;
use crate::objects::rectangle::{XYRect, XZRect, YZRect};

pub struct AxisAlignedBox {
    pub box_min: Point,
    pub box_max: Point,
    pub faces: HittableList,
}

impl AxisAlignedBox {
    pub fn new(point_min: Point, point_max: Point, mat: Arc<dyn Material>) -> Self {
        let mut faces: Vec<Arc<dyn Hittable>> = Vec::new();
        faces.push(Arc::new(XYRect {
            x: (point_min.x, point_max.x),
            y: (point_min.y, point_max.y),
            k: point_max.z,
            material: mat.clone(),
        }));
        faces.push(Arc::new(XYRect {
            x: (point_min.x, point_max.x),
            y: (point_min.y, point_max.y),
            k: point_min.z,
            material: mat.clone(),
        }));


        faces.push(Arc::new(XZRect {
            x: (point_min.x, point_max.x),
            z: (point_min.z, point_max.z),
            k: point_min.y,
            material: mat.clone(),
        }));
        faces.push(Arc::new(XZRect {
            x: (point_min.x, point_max.x),
            z: (point_min.z, point_max.z),
            k: point_max.y,
            material: mat.clone(),
        }));

        faces.push(Arc::new(YZRect {
            y: (point_min.y, point_max.y),
            z: (point_min.z, point_max.z),
            k: point_max.x,
            material: mat.clone(),
        }));
        faces.push(Arc::new(YZRect {
            y: (point_min.y, point_max.y),
            z: (point_min.z, point_max.z),
            k: point_min.x,
            material: mat.clone(),
        }));

        return Self {
            box_min: point_min,
            box_max: point_max,
            faces: HittableList { objects: faces },
        };
    }
}

impl Hittable for AxisAlignedBox {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        return self.faces.hit(ray, t_min, t_max);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AxisAlignedBoundingBox> {
        return Option::from(AxisAlignedBoundingBox {
            minimum: self.box_min,
            maximum: self.box_max,
        });
    }
}