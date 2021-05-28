use crate::geometry::ray::Ray;
use crate::geometry::vector::{Point, Vector3};

pub trait Hittable {
    fn hit(&self, _ray: &Ray, _t_min: f32, _t_max: f32) -> f32;
    fn normal(&self, _intersection: Point) -> Vector3;
}

pub struct HitRecord<'a> {
    pub object: Option<&'a Box<dyn Hittable>>,
    pub intersection: Point,
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> HitRecord {
        let mut closest_distance: f32 = t_max;
        let mut nearest_object: Option<&Box<dyn Hittable>> = Option::None;

        for object in &self.objects {
            let distance = object.hit(ray, t_min, t_max);
            if distance > 0.0 && distance < closest_distance {
                closest_distance = distance;
                nearest_object = Option::from(object);
            }
        }

        HitRecord {
            object: nearest_object,
            intersection: ray.at_distance(closest_distance),
        }
    }
}