use rand::Rng;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.
}
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
pub fn random_double(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min, max)
}
