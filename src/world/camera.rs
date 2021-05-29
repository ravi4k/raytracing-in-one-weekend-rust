use crate::geometry::vector::{Point, Vector3};
use crate::geometry::ray::Ray;
use crate::utils::degrees_to_radians;

pub struct Camera {
    pub position: Point,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub lower_left_corner: Point,
}

impl Camera {
    pub fn new(look_from: Point, look_at: Point, v_up: Vector3, v_fov: f32, aspect_ratio: f32) -> Self {
        let theta = degrees_to_radians(v_fov);
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = viewport_height * aspect_ratio;

        let normal_in = (look_from - look_at).direction();
        let horizontal_orientation = v_up.cross(normal_in).direction();
        let vertical_orientation = normal_in.cross(horizontal_orientation);

        let position = look_from;
        let horizontal = viewport_width * horizontal_orientation;
        let vertical = viewport_height * vertical_orientation;
        let lower_left_corner = position - horizontal / 2.0 - vertical / 2.0 - normal_in;

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