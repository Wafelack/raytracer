use std::rc::Rc;
use std::{io, io::Write};

mod camera;
mod vec3;
mod colors; 
mod objects;
mod utils;
mod ray;
mod material;

use camera::Camera;
use vec3::*;
use colors::write_color;
use objects::{hittable::*, hittable_list::*, sphere::*};
use utils::*;
use ray::*;
use material::material::*;



fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1024;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 8;
    const MAX_DEPTH: i32 = 50;

    // World

    let mut world = HittableList::new();

    let material_ground :Rc<dyn Material>= Rc::new(Lambertian::from(color::from(0.8,0.8,0.)));
    let material_center :Rc<dyn Material>= Rc::new(Lambertian::from(color::from(0.1,0.2,0.5)));
    let material_left :Rc<dyn Material>= Rc::new(Dielectric::new(1.5));
    let material_right :Rc<dyn Material>= Rc::new(Metal::from(color::from(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(point3::from(0., -100.5, -1.),100., material_ground)));
    world.add(Rc::new(Sphere::new(point3::from(0.,0.,-1.), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(point3::from(-1.,0.,-1.), 0.5, Rc::clone(&material_left))));
    world.add(Rc::new(Sphere::new(point3::from(-1.,0.,-1.), -0.45, Rc::clone(&material_left))));
    world.add(Rc::new(Sphere::new(point3::from(1.,0.,-1.), 0.5, material_right)));
    // Camera
    let cam = Camera::new(point3::from(-2.,2.,1.), point3::from(0.,0.,-1.), Vec3::from(0.,1.,0.), 90.0, ASPECT_RATIO);

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}/{}            ", j, IMAGE_HEIGHT);
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
