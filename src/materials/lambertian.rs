use crate::geometry::color::Color;
use crate::geometry::vector::Point;
use crate::materials::material::Material;
use crate::textures::texture::Texture;
use std::sync::Arc;
use crate::geometry::ray::Ray;
use crate::objects::hittable::HitRecord;
use crate::textures::solid::SolidColor;
use crate::utils::PI;
use crate::geometry::onb::ONB;
use crate::geometry::pdf::random_cosine_direction;

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
    fn scatter(&self, in_ray: Ray, hit_rec: &HitRecord) -> Option<Ray> {
        let uvw = ONB::build_from_w(hit_rec.normal);
        let mut scatter_direction = uvw.local(random_cosine_direction());
        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }
        return Option::from(Ray {
            origin: hit_rec.intersection,
            direction: scatter_direction.direction(),
            time: in_ray.time,
        });
    }

    fn pdf(&self, _in_ray: Ray, hit_rec: &HitRecord, scattered_ray: Ray) -> f32 {
        return hit_rec.normal.dot(scattered_ray.direction) / PI;
    }

    fn scattering_pdf(&self, _in_ray: Ray, hit_rec: &HitRecord, scattered_ray: Ray) -> f32 {
        let cosine = hit_rec.normal.dot(scattered_ray.direction);
        return if cosine < 0.0 { 0.0 } else { cosine / PI };
    }

    fn color(&self, u: f32, v: f32, intersection: Point) -> Color {
        self.albedo.color(u, v, intersection)
    }
}
