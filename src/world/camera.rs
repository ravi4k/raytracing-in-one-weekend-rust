use crate::geometry::ray::Ray;
use crate::geometry::vector::{Point, Vector3};
use crate::utils::{degrees_to_radians, random_f32_range, random_in_unit_disk};

#[derive(Clone)]
pub struct Screen {
    width: Vector3,
    height: Vector3,
    upper_left_corner: Point,
}

impl Screen {
    fn pixel_position(&self, x: f32, y: f32) -> Point {
        self.upper_left_corner + x * self.width - y * self.height
    }
}

#[derive(Clone)]
pub struct Camera {
    position: Point,
    horizontal_orientation: Vector3,
    vertical_orientation: Vector3,
    aperture: f32,
    viewport: Screen,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(look_from: Point, look_at: Point, v_up: Vector3, v_fov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32, time0: f32, time1: f32) -> Self {
        let normal_in = (look_from - look_at).direction();
        let horizontal_orientation = v_up.cross(normal_in).direction();
        let vertical_orientation = normal_in.cross(horizontal_orientation);

        let viewport = Self::setup_viewport(look_from, normal_in, horizontal_orientation, vertical_orientation, v_fov, aspect_ratio, focus_dist);

        Camera {
            position: look_from,
            horizontal_orientation,
            vertical_orientation,
            aperture,
            viewport,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.aperture * random_in_unit_disk();
        let offset = rd.x * self.horizontal_orientation + rd.y * self.vertical_orientation;

        let ray_origin = self.position + offset;
        let ray_direction = (self.viewport.pixel_position(u, v) - ray_origin).direction();
        Ray {
            origin: ray_origin,
            direction: ray_direction,
            time: random_f32_range(self.time0, self.time1),
        }
    }

    fn setup_viewport(position: Point, normal_in: Point, horizontal_orientation: Vector3, vertical_orientation: Point, v_fov: f32, aspect_ratio: f32, focus_dist: f32) -> Screen {
        let theta = degrees_to_radians(v_fov);
        let height_scalar = 2.0 * (theta / 2.0).tan();
        let width_scalar = height_scalar * aspect_ratio;

        let dist_from_camera = focus_dist * normal_in;
        let width = focus_dist * width_scalar * horizontal_orientation;
        let height = focus_dist * height_scalar * vertical_orientation;
        let upper_left_corner = position - width / 2.0 + height / 2.0 - dist_from_camera;

        Screen {
            width,
            height,
            upper_left_corner,
        }
    }
}