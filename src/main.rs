use image::{RgbImage, ImageBuffer};

mod geometry;
use geometry::vector::{Vector3, Point};
use geometry::color::Color;
use geometry::ray::Ray;

mod utils;

fn hit_sphere(center: Point, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * c;

    if discriminant > 0.0 {
        return (-b - discriminant.sqrt()) / 2.0;
    }
    -1.0
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(Point { x: 0.0, y: 0.0, z: -1.0 }, 0.5, &ray);

    if t > 0.0 {
        let normal = (ray.at_distance(t) - Point { x: 0.0, y: 0.0, z: -1.0 }).direction();
        return 0.5 * Color {r: normal.x + 1.0, g: normal.y + 1.0, b: normal.z + 1.0};
    }

    let t = 0.5 * (ray.direction.y + 1.0);
    (1.0 - t) * Color {r: 1.0, g: 1.0, b: 1.0} + t * Color {r: 0.5, g: 0.7, b: 1.0}
}

fn main() {

    // Image
    const IMAGE_WIDTH: u32 = 1280;
    const IMAGE_HEIGHT: u32 = 720;
    const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;

    //Camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f32 = 1.0;

    let cam_position: Point = Point { x: 0.0, y: 0.0, z:0.0 };
    let horizontal: Point = Point { x: VIEWPORT_WIDTH, y: 0.0, z: 0.0 };
    let vertical: Point = Point { x: 0.0, y: VIEWPORT_HEIGHT, z: 0.0 };
    let lower_left: Point = cam_position - horizontal / 2.0 - vertical / 2.0 - Point { x: 0.0, y: 0.0, z: FOCAL_LENGTH };

    // Render
    let mut img_buf: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev()  {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32/ (IMAGE_WIDTH - 1) as f32;
            let v = j as f32/ (IMAGE_HEIGHT - 1) as f32;

            let ray_direction = (lower_left + u * horizontal + v * vertical - cam_position).direction();
            let ray = Ray {
                origin: cam_position,
                direction: ray_direction,
            };

            let pixel_color: Color = ray_color(ray);
            img_buf.put_pixel(i,IMAGE_HEIGHT - 1 - j, pixel_color.get_pixel());
        }
    }
    img_buf.save("render.png").unwrap();
}
