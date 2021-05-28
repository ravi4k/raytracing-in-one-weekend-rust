use rand::distributions::{Distribution, Uniform};
use rand::{Rng, thread_rng};

// Constants
pub const INF_F32: f32 = f32::MAX;
pub const PI: f32 = std::f32::consts::PI;

// Functions
pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

pub fn random_f32() -> f32 {
    thread_rng().gen()
}

pub fn random_f32_range(min: f32, max: f32) -> f32 {
    random_f32() * (max - min) + min
}
