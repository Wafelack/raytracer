use std::sync::Arc;

mod aabb;
mod bvh;
mod camera;
mod canvas;
mod colors;
mod material;
mod objects;
mod ray;
mod texture;
mod utils;
mod vec3;

pub use camera::Camera;
pub use canvas::Canvas;
pub use material::material::*;
pub use objects::{hittable_list::*, moving_sphere::*, sphere::*};
pub use ray::*;
pub use texture::*;
pub use utils::*;
pub use vec3::*;

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = Arc::new(CheckerTexture::from_colors(
        color::from(0.2, 0.3, 0.1),
        color::from(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere::new(
        point3::from(0., -10., 0.),
        10.,
        Arc::new(Lambertian::from_texture(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        point3::from(0., 10., 0.),
        10.,
        Arc::new(Lambertian::from_texture(checker.clone())),
    )));

    objects
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        color::from(0.2, 0.3, 0.1),
        color::from(0.9, 0.9, 0.9),
    ));

    let ground_material = Arc::new(Lambertian::from_texture(checker));
    world.add(Arc::new(Sphere::new(
        point3::from(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0., 1.);
            let center = point3::from(
                a as f32 + 0.9 * random_double(0., 1.),
                0.2,
                b as f32 + 0.9 * random_double(0., 1.),
            );

            if (center - point3::from(4., 0.2, 0.)).len() > 0.9 {
                let mut sphere_material: Arc<dyn Material> =
                    Arc::new(Metal::from(color::new(), 0.0));

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = color::random(0., 1.) * color::random(0., 1.);
                    sphere_material = Arc::new(Lambertian::from(albedo));
                    let center2 = center + Vec3::from(0., random_double(0., 0.5), 0.);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.,
                        1.,
                        0.2,
                        sphere_material,
                    )));
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
    world.add(Arc::new(Sphere::new(
        point3::from(0., 1., 0.),
        1.,
        material1,
    )));

    let material2 = Arc::new(Lambertian::from(color::from(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        point3::from(-4., 1., 0.),
        1.,
        material2,
    )));

    let material3 = Arc::new(Metal::from(color::from(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        point3::from(4., 1., 0.),
        1.,
        material3,
    )));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 300;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 50;
    const MAX_DEPTH: usize = 50;

    // World

    let world = HittableList::new();

    // Camera

    let mut lookfrom = point3::from(13., 2., 3.);
    let mut lookat = point3::from(0., 0., 0.);
    let mut vup = Vec3::from(0., 1., 0.);
    let mut dist_to_focus: f32 = 10.0;
    let mut aperture: f32 = 0.1;
    let mut vfov = 40.;

    let mode = 0;

    match mode {
        0 => {
            world = two_spheres();
            lookfrom = point3::from(13., 2., 3.);
            lookat = point3::new(); // 0, 0 and 0
            vfov = 20.;
        }
        _ => {
            world = random_scene();
        }
    }

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.,
        1.,
    );

    let render_pixel = |i, j| -> color {
        let mut pixel_color = color::new();
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (i as f32 + random_double(0., 1.)) / ((IMAGE_WIDTH - 1) as f32);
            let v = (j as f32 + random_double(0., 1.)) / ((IMAGE_HEIGHT - 1) as f32);
            let r = cam.get_ray(u, v);
            pixel_color.add(ray_color(r, &*world, MAX_DEPTH as i32));
        }
        pixel_color
    };

    // Render
    let c = Canvas::from_fn_parallel_with_progress(
        IMAGE_WIDTH as usize,
        IMAGE_HEIGHT as usize,
        SAMPLES_PER_PIXEL as usize,
        render_pixel,
        |total, num_done| {
            eprint!(
                "\r{:.2}% done",
                ((num_done as f32 / total as f32) * 100.).min(100.)
            );
        },
    );
    c.write_header();
    c.write_pixels();
    eprint!("\nDone\n");
}
