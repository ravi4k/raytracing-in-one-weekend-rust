use std::sync::Arc;

use crate::geometry::color::Color;
use crate::geometry::ray::Ray;
use crate::geometry::vector::{Point, Vector3};
use crate::materials::material::{Material, ScatterRecord};
use crate::objects::hittable::HitRecord;
use crate::textures::texture::Texture;

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}

impl Material for Isotropic {
    fn scatter(&self, in_ray: Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        return Option::from(ScatterRecord {
            specular_ray: Option::from(Ray { origin: hit_rec.intersection, direction: Vector3::random_unit_vector(), time: in_ray.time }),
            attenuation: self.albedo.color(hit_rec.u, hit_rec.v, hit_rec.intersection),
            pdf_ptr: None,
        });
    }
}
