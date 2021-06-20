use std::sync::Arc;

use crate::geometry::color::Color;
use crate::geometry::onb::ONB;
use crate::geometry::pdf::{CosinePDF, PDF, random_cosine_direction};
use crate::geometry::ray::Ray;
use crate::geometry::vector::Point;
use crate::materials::material::{Material, ScatterRecord};
use crate::objects::hittable::HitRecord;
use crate::textures::solid::SolidColor;
use crate::textures::texture::Texture;
use crate::utils::PI;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        return Self {
            albedo: Arc::new(SolidColor { color })
        };
    }
}

impl Material for Lambertian {
    fn scatter(&self, in_ray: Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        return Option::from(ScatterRecord {
            specular_ray: None,
            attenuation: self.albedo.color(hit_rec.u, hit_rec.v, hit_rec.intersection),
            pdf_ptr: Option::from(Arc::new(CosinePDF::new(hit_rec.normal)) as Arc<dyn PDF>),
        });
    }

    fn scattering_pdf(&self, _in_ray: Ray, hit_rec: &HitRecord, scattered_ray: Ray) -> f32 {
        let cosine = hit_rec.normal.dot(scattered_ray.direction);
        return if cosine < 0.0 { 0.0 } else { cosine / PI };
    }
}
