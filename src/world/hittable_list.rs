use crate::objects::hittable::{Hittable, HitRecord};
use std::sync::Arc;
use crate::geometry::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn hit(&self, in_ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_rec: Option<HitRecord> = Option::None;
        let mut closest = t_max;

        for obj in self.objects.iter() {
            let opt_temp_rec = obj.hit(in_ray, t_min, t_max);
            if opt_temp_rec.is_some() {
                let temp_rec = opt_temp_rec.unwrap();
                if temp_rec.t < closest {
                    closest = temp_rec.t;
                    hit_rec = Option::from(temp_rec);
                }
            }
        }
        return hit_rec;
    }
}
