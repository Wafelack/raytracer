use std::rc::Rc;
use std::{io, io::Write};
mod camera;
use camera::Camera;
mod vec3;
use vec3::*;
mod colors;
use colors::write_color;
mod objects;
mod utils;
use objects::{hittable::*, hittable_list::*, sphere::*};
use utils::*;
mod ray;
use ray::*;

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
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 8;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(point3::from(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(point3::from(0., -100.5, -1.), 100.)));

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = color::new();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random_double(0., 1.)) / ((IMAGE_WIDTH - 1) as f32);
                let v = (j as f32 + random_double(0., 1.)) / ((IMAGE_HEIGHT - 1) as f32);
                let r = cam.get_ray(u, v);
                pixel_color.add(ray_color(r, &world));
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone\n");
    io::stdout().flush().unwrap();
}
