use crate::utils::clamp;
use crate::vec3::*;

pub fn write_color(pixel_color: color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the samples amount
    let scale = 1. / samples_per_pixel as f32;

    r *= scale;
    g *= scale;
    b *= scale;

    println!(
        "{} {} {}",
        (256. * clamp(r, 0., 0.999)) as i32,
        (256. * clamp(g, 0., 0.999)) as i32,
        (256. * clamp(b, 0., 0.999)) as i32
    );
}
