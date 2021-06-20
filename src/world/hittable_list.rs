use std::sync::Arc;

use crate::geometry::ray::Ray;
use crate::geometry::vector::{Point, Vector3};
use crate::objects::hittable::{HitRecord, Hittable};
use crate::utils::random_int;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        return Self { objects: Vec::new() };
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, in_ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

    fn pdf_value(&self, o: Point, v: Vector3) -> f32 {
        let weight = 1.0 / self.objects.len() as f32;
        let mut sum = 0.0;

        for obj in self.objects.iter() {
            sum += weight * obj.pdf_value(o, v);
        }

        return sum;
    }

    fn random(&self, o: Vector3) -> Vector3 {
        let int_size = self.objects.len() as i32;
        return self.objects[random_int(0, (int_size - 1) as u32) as usize].random(o);
    }
}
