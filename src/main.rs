use image::{RgbImage, ImageBuffer};

mod geometry;
use geometry::vector::{Vector3, Point};
use geometry::color::Color;
use geometry::ray::Ray;

mod objects;
use objects::hittable::{HitRecord, HittableList};
use objects::sphere::Sphere;

mod world;
use world::camera::Camera;

mod utils;
use utils::{INF_F32, PI, degrees_to_radians};
use crate::utils::random_f32;

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    let hit_rec = world.hit(&ray, 0.0, INF_F32);
    if hit_rec.object.is_some() {
        let normal = hit_rec.object.unwrap().normal(hit_rec.intersection);
        return 0.5 * Color {
            r: normal.x + 1.0,
            g: normal.y + 1.0,
            b: normal.z + 1.0,
        };
    }

    let t = 0.5 * (ray.direction.y + 1.0);
    (1.0 - t) * Color {r: 1.0, g: 1.0, b: 1.0} + t * Color {r: 0.5, g: 0.7, b: 1.0}
}

fn main() {

    // Image
    const IMAGE_WIDTH: u32 = 640;
    const IMAGE_HEIGHT: u32 = 480;
    const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
    const SPP: u32 = 10;

    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.add(Box::new(Sphere {
        center: Point { x: 0.0, y: 0.0, z: -1.0},
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point { x: 0.0, y: -100.5, z: -1.0},
        radius: 100.0,
    }));

    //Camera
    let camera = Camera::new();

    // Render
    let mut img_buf: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev()  {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color { r: 0.0, g: 0.0, b: 0.0};
            for _ in 0..SPP {
                let u = (i as f32 + random_f32()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + random_f32()) / (IMAGE_HEIGHT - 1) as f32;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, &world);
            }
            img_buf.put_pixel(i, IMAGE_HEIGHT - 1 - j, pixel_color.get_pixel(SPP));
        }
    }
    img_buf.save("render.png").unwrap();
}