use std::sync::Arc;
use std::{io, io::Write};

mod camera;
mod vec3;
mod colors; 
mod objects;
mod utils;
mod ray;
mod material;
mod canvas;

use camera::Camera;
use vec3::*;
use colors::write_color;
use objects::{moving_sphere::*, hittable_list::*, sphere::*};
use utils::*;
use ray::*;
use material::material::*;
use canvas::Canvas;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::from(color::from(0.5,0.5,0.5)));
    world.add(Arc::new(Sphere::new(point3::from(0.,-1000.,0.), 1000., ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0., 1.);
            let center = point3::from(a as f32 + 0.9*random_double(0., 1.), 0.2, b as f32+ 0.9*random_double(0., 1.));
            
            if (center - point3::from(4.,0.2, 0.)).len() > 0.9 {
                let mut sphere_material: Arc<dyn Material> = Arc::new(Metal::from(color::new(), 0.0));

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = color::random(0., 1.) * color::random(0., 1.);
                    sphere_material = Arc::new(Lambertian::from(albedo));
                    let center2 = center + Vec3::from(0., random_double(0., 0.5), 0.);
                    world.add(Arc::new(MovingSphere::new(center, center2,0.,1., 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = color::random(0.5, 1.);
                    let fuzz = random_double(0., 0.5);
                    sphere_material = Arc::new(Metal::from(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }

        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(point3::from(0., 1., 0.), 1., material1)));

    let material2 = Arc::new(Lambertian::from(color::from(0.4,0.2,0.1)));
    world.add(Arc::new(Sphere::new(point3::from(-4., 1., 0.), 1., material2)));

    let material3 = Arc::new(Metal::from(color::from(0.7,0.6,0.5), 0.0));
    world.add(Arc::new(Sphere::new(point3::from(4., 1., 0.), 1., material3)));

    world

}


fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1000;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World

    let world = Arc::new(random_scene());

    // Camera

    let lookfrom = point3::from(13.,2.,3.);
    let lookat = point3::from(0.,0.,0.);
    let vup = Vec3::from(0.,1.,0.);
    let dist_to_focus: f32 = 10.0;
    let aperture: f32 = 0.1;


    let cam = Camera::new(lookfrom, lookat, vup, 20., ASPECT_RATIO, aperture, dist_to_focus, 0., 1.);

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let c = Canvas::from_fn_paralell_with_progress(IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize , SAMPLES_PER_PIXEL as usize , |i , j|{
        let mut pixel_color = color::new();
        for _ in 0..SAMPLES_PER_PIXEL{
                let u = (i as f32 + random_double(0., 1.)) / ((IMAGE_WIDTH - 1) as f32);
                let v = (j as f32 + random_double(0., 1.)) / ((IMAGE_HEIGHT - 1) as f32);
                let r = cam.get_ray(u, v);
                pixel_color.add(ray_color(r, &*world, MAX_DEPTH));
        }
        pixel_color
    } , |total , num_done|{
        eprint!("\r{:.2}" , ((num_done as f32 / total as f32) * 100.).min(100.));
    });
    c.write_pixels();
    eprint!("\nDone\n");
}
