use std::rc::Rc;
use std::{io, io::Write};

mod vec3;
use vec3::*;

mod utils;
use utils::*;

mod objects;
use objects::{hittable::*, hittable_list::*, sphere::*};

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
    let a = r.direction().len_squared();
    let half_b = dot(oc, r.direction());
    let c = oc.len_squared() - radius * radius;
    let discrimninant = half_b * half_b - a * c;
    if discrimninant < 0. {
        return -1.;
    } else {
        return (-half_b - discrimninant.sqrt()) / a;
    }
}
fn ray_color(r: Ray, world: &impl Hittable) -> color {
    let mut rec = HitRecord::void();

    if world.hit(r, 0., f32::INFINITY, &mut rec) {
        return (rec.normal + color::from(1., 1., 1.)) * 0.5;
    }
    let unit_direction = unit_vector(r.direction());

    let t = (unit_direction.y() + 1.) * 0.5;

    color::from(1., 1., 1.) * (1. - t) + color::from(0.5, 0.7, 1.) * t
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 2048;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(point3::from(-0.6, 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(point3::from(0.6, 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(point3::from(0., -100.5, -1.), 100.)));

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
            let pixel_color = ray_color(r, &world);
            write_color(pixel_color);
        }
    }
    eprint!("\nDone\n");
    io::stdout().flush().unwrap();
}
