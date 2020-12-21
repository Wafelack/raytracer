use rand::Rng;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees.to_radians()
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
    // TODO likely a huge bottleneck especially for multi thread
    rand::thread_rng().gen_range(min, max)
}

pub fn random_int(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min, max)
}
