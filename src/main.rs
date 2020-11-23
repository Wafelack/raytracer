use std::{io, io::Write};

mod vec3;
use vec3::*;

mod ray;
use ray::*;

fn write_color(pixel_color: color) {
    println!(
        "{} {} {}",
        (pixel_color.x() * 255.99) as i32,
        (pixel_color.y() * 255.99) as i32,
        (pixel_color.z() * 255.99) as i32
    );
}

fn hit_sphere(center: point3, radius: f32, r: Ray) -> f32 {
    let oc = r.origin() - center;
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(oc, r.direction());
    let c = dot(oc, oc) - radius * radius;
    let discrimninant = b * b - 4. * a * c;
    if discrimninant < 0. {
        return -1.;
    } else {
        return (-b - discrimninant.sqrt()) / (2. * a);
    }
}
fn ray_color(r: Ray) -> color {
    let t = hit_sphere(point3::from(0., 0., -1.), 0.5, r);
    if t > 0. {
        let N = unit_vector(r.at(t) - Vec3::from(0., 0., -1.));
        return color::from(N.x() + 1., N.y() + 1., N.z() + 1.) * 0.5;
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    color::from(1.0, 1.0, 1.0) * (1.0 - t) + color::from(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 32.0 / 18.0;
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = point3::from(0., 0., 0.);
    let horizontal = Vec3::from(viewport_width, 0., 0.);
    let vertical = Vec3::from(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::from(0., 0., focal_length);
    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / ((IMAGE_WIDTH - 1) as f32);
            let v = j as f32 / ((IMAGE_HEIGHT - 1) as f32);
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(r);
            write_color(pixel_color);
        }
    }
    eprint!("\nDone\n");
    io::stdout().flush().unwrap();
}
