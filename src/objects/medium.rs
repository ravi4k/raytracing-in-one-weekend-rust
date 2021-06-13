use crate::objects::hittable::{Hittable, HitRecord};
use std::sync::Arc;
use crate::materials::material::Material;
use crate::geometry::bounding_volume::AxisAlignedBoundingBox;
use crate::geometry::ray::Ray;
use crate::utils::{INF_F32, random_f32};
use crate::geometry::vector::Vector3;
use crate::geometry::color::Color;
use crate::materials::isotropic::Isotropic;
use crate::textures::solid::SolidColor;

pub struct ConstMedium {
    pub boundary: Arc<dyn Hittable>,
    pub phase_function: Arc<dyn Material>,
    pub neg_inv_density: f32,
}

impl ConstMedium {
    pub fn new(boundary: Arc<dyn Hittable>, d: f32, color: Color) -> Self {
        return Self {
            boundary,
            phase_function: Arc::new(Isotropic { color: Arc::new(SolidColor { color }) }),
            neg_inv_density: -1.0 / d,
        };
    }
}

impl Hittable for ConstMedium {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let opt_hit_rec1 = self.boundary.hit(ray, -INF_F32, INF_F32);
        if opt_hit_rec1.is_none() {
            return opt_hit_rec1;
        }
        let mut hit_rec1 = opt_hit_rec1.unwrap();

        let opt_hit_rec2 = self.boundary.hit(ray, hit_rec1.t + 0.0001, INF_F32);
        if opt_hit_rec2.is_none() {
            return opt_hit_rec2;
        }
        let mut hit_rec2 = opt_hit_rec2.unwrap();

        if hit_rec1.t < t_min { hit_rec1.t = t_min }
        if hit_rec2.t > t_max { hit_rec2.t = t_max }
        if hit_rec1.t >= hit_rec2.t {
            return Option::None;
        }

        if hit_rec1.t < 0.0 { hit_rec1.t = 0.0 }

        let dist_inside_boundary = hit_rec2.t - hit_rec1.t;
        let hit_dist = self.neg_inv_density * random_f32().ln();

        if hit_dist > dist_inside_boundary {
            return Option::None;
        }

        let t = hit_rec1.t + hit_dist;
        return Option::from(HitRecord {
            intersection: ray.at_distance(t),
            normal: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            material: self.phase_function.clone(),
            t,
            u: 0.0,
            v: 0.0,
            front_face: true
        });
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AxisAlignedBoundingBox> {
        return self.boundary.bounding_box(t0, t1);
    }
}
