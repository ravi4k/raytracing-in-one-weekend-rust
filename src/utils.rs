use rand::{thread_rng, Rng};
use crate::geometry::vector::{Vector3, Point};

// Constants
pub const INF_F32: f32 = f32::MAX;
pub const PI: f32 = std::f32::consts::PI;

// Functions
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_f32() -> f32 {
    thread_rng().gen()
}

pub fn random_f32_range(min: f32, max: f32) -> f32 {
    random_f32() * (max - min) + min
}

pub fn random_in_unit_sphere() -> Point {
    let direction = Vector3::random_unit_vector();
    let distance = random_f32().cbrt();
    distance * direction
}

pub fn random_in_unit_disk() -> Point {
    let direction = Vector3::random_unit_vector();
    let distance = random_f32().sqrt();
    Point {
        x: distance * direction.x,
        y: distance * direction.y,
        z: 0.0,
    }
}
