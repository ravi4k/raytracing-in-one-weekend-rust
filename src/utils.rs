use rand::{Rng, thread_rng};

use crate::geometry::vector::{Point, Vector3};

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

pub fn random_int(min: u32, max: u32) -> u32 {
    thread_rng().gen_range(min..(max + 1))
}

pub fn random_to_sphere(radius: f32, dist_sq: f32) -> Vector3 {
    let r1 = random_f32();
    let r2 = random_f32();
    let z = 1.0 + r2 * ((1.0 - radius.powi(2) / dist_sq).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z.powi(2)).sqrt();
    let y = phi.sin() * (1.0 - z.powi(2)).sqrt();

    return Vector3 { x, y, z };
}