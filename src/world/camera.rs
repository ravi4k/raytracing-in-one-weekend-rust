use crate::geometry::vector::{Point, Vector3};
use crate::geometry::ray::Ray;

pub struct Camera {
    pub position: Point,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub lower_left_corner: Point,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let position = Point { x: 0.0, y: 0.0, z: 0.0 };
        let horizontal = Vector3 { x: viewport_width, y: 0.0, z: 0.0};
        let vertical = Vector3 { x: 0.0, y: viewport_height, z: 0.0};
        let lower_left_corner = position - horizontal / 2.0 - vertical / 2.0 - Vector3 {x: 0.0, y: 0.0, z: focal_length};

        Camera {
            position,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let ray_direction = (self.lower_left_corner + u * self.horizontal + v * self.vertical - self.position).direction();
        Ray {
            origin: self.position,
            direction: ray_direction,
        }
    }
}