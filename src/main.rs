use std::rc::Rc;
use std::{io, io::Write};

mod camera;
mod vec3;
mod colors; 
mod objects;
mod utils;
mod ray;

use camera::Camera;
use vec3::*;
use colors::write_color;
use objects::{hittable::*, hittable_list::*, sphere::*};
use utils::*;
use ray::*;



fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 512;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 9;
    const MAX_DEPTH: i32 = 50;

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
                pixel_color.add(ray_color(r, &world, MAX_DEPTH));
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone\n");
    io::stdout().flush().unwrap();
}
