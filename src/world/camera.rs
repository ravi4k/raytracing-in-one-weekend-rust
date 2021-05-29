use crate::geometry::vector::{Point, Vector3};
use crate::geometry::ray::Ray;
use crate::utils::{degrees_to_radians, random_in_unit_disk};

pub struct Camera {
    pub position: Point,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub lower_left_corner: Point,
    pub horizontal_orientation: Vector3,
    pub vertical_orientation: Vector3,
    pub aperture: f32,
}

impl Camera {
    pub fn new(look_from: Point, look_at: Point, v_up: Vector3, v_fov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        let theta = degrees_to_radians(v_fov);
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = viewport_height * aspect_ratio;

        let normal_in = (look_from - look_at).direction();
        let horizontal_orientation = v_up.cross(normal_in).direction();
        let vertical_orientation = normal_in.cross(horizontal_orientation);

        let position = look_from;
        let horizontal = focus_dist * viewport_width * horizontal_orientation;
        let vertical = focus_dist * viewport_height * vertical_orientation;
        let lower_left_corner = position - horizontal / 2.0 - vertical / 2.0 - focus_dist * normal_in;

        Camera {
            position,
            horizontal,
            vertical,
            lower_left_corner,
            horizontal_orientation,
            vertical_orientation,
            aperture,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.aperture * random_in_unit_disk();
        let offset = rd.x * self.horizontal_orientation + rd.y * self.vertical_orientation;
        let ray_direction = (self.lower_left_corner + u * self.horizontal + v * self.vertical - self.position - offset).direction();
        Ray {
            origin: self.position + offset,
            direction: ray_direction,
        }
    }
}