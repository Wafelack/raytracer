use std::sync::Arc;
use std::time::Instant;

mod aabb;
mod aarect;
mod bvh;
mod camera;
mod canvas;
mod colors;
mod material;
mod objects;
mod perlin;
mod ray;
mod texture;
mod utils;
mod vec3;

pub use aarect::*;
pub use camera::Camera;
pub use canvas::Canvas;
pub use material::boxx::*;
pub use material::material::*;
pub use objects::{hittable_list::*, moving_sphere::*, sphere::*};
pub use perlin::*;
pub use ray::*;
pub use texture::*;
pub use utils::*;
pub use vec3::*;

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::from(color::from(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(color::from(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(color::from(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(color::from(30., 30., 30.)));

    objects.add(Arc::new(YzRect::from(0., 555., 0., 555., 555., green)));
    objects.add(Arc::new(YzRect::from(0., 555., 0., 555., 0., red)));
    objects.add(Arc::new(XzRect::from(213., 343., 227., 332., 554., light)));
    objects.add(Arc::new(XzRect::from(
        0.,
        555.,
        0.,
        555.,
        0.,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::from(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::from(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    objects.add(Arc::new(Boxx::from(
        &point3::from(130., 0., 65.),
        &point3::from(295., 165., 230.),
        white.clone(),
    )));
    objects.add(Arc::new(Boxx::from(
        &point3::from(265., 0., 295.),
        &point3::from(430., 330., 460.),
        white.clone(),
    )));

    objects
}

fn simple_light() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture::from(4.));
    objects.add(Arc::new(Sphere::new(
        point3::from(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        point3::from(0., 2., 0.),
        2.,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));

    let difflight = Arc::new(DiffuseLight::from_color(color::from(4., 4., 4.)));
    objects.add(Arc::new(XyRect::from(3., 5., 1., 3., -2., difflight)));

    objects
}

fn custom_scene() -> HittableList {
    let mut objects = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::from("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Arc::new(Sphere::new(point3::from(0., 2., 0.), 2., earth_surface));

    objects.add(globe);
    let difflight = Arc::new(DiffuseLight::from_color(color::from(4., 4., 4.)));
    objects.add(Arc::new(XyRect::from(3., 5., 1., 3., -2., difflight)));

    objects
}

fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::from("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Arc::new(Sphere::new(point3::new(), 2., earth_surface));

    let mut objects = HittableList::new();
    objects.add(globe);
    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture::from(4.));
    objects.add(Arc::new(Sphere::new(
        point3::from(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        point3::from(0., 2., 0.),
        2.,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));

    objects
}

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
    let time = Instant::now(); // Time counter
                               // Image
    let mut aspect_ratio: f32 = 16.0 / 9.0;
    let mut image_width: usize = 400;

    let mut samples_per_pixel: usize = 100;
    const MAX_DEPTH: usize = 50;

    // World

    let mut world = HittableList::new();

    // Camera

    let mut lookfrom = point3::from(13., 2., 3.);
    let mut lookat = point3::from(0., 0., 0.);
    let mut vup = Vec3::from(0., 1., 0.);
    let mut dist_to_focus: f32 = 10.0;
    let mut aperture: f32 = 0.0;
    let mut vfov = 40.;
    let mut background = color::new();

    let mode = 6;

    match mode {
        0 => {
            world = two_spheres();
            lookfrom = point3::from(13., 2., 3.);
            lookat = point3::new(); // 0, 0 and 0
            vfov = 20.;
        }
        1 => {
            world = two_perlin_spheres();
            lookfrom = point3::from(13., 2., 3.);
            lookat = point3::new(); // 0, 0 and 0
            vfov = 20.;
        }
        2 => {
            world = earth();
            lookfrom = point3::from(13., 2., 3.);
            lookat = point3::new(); // 0, 0 and 0
            vfov = 20.;
            background = color::from(1., 1., 1.);
        }
        3 => {}
        4 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = color::new();
            lookfrom = point3::from(26., 3., 6.);
            lookat = point3::from(0., 2., 0.);
            vfov = 20.;
        }
        5 => {
            world = custom_scene();
            samples_per_pixel = 400;
            background = color::new();
            lookfrom = point3::from(26., 3., 6.);
            lookat = point3::from(0., 2., 0.);
            vfov = 20.;
        }
        6 => {
            world = cornell_box();
            aspect_ratio = 1.;
            image_width = 600;
            samples_per_pixel = 500;
            background = color::new();
            lookfrom = point3::from(278., 278., -800.);
            lookat = point3::from(278., 278., 0.);
            vfov = 40.;
        }
        _ => {
            world = random_scene();
        }
    }

    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.,
        1.,
    );

    let render_pixel = |i, j| -> color {
        let mut pixel_color = color::new();
        for _ in 0..samples_per_pixel {
            let u = (i as f32 + random_double(0., 1.)) / ((image_width - 1) as f32);
            let v = (j as f32 + random_double(0., 1.)) / ((image_height - 1) as f32);
            let r = cam.get_ray(u, v);
            pixel_color.add(ray_color(r, &background, &world, MAX_DEPTH as i32));
        }
        pixel_color
    };

    // Render
    let c = Canvas::from_fn_parallel_with_progress(
        image_width as usize,
        image_height as usize,
        samples_per_pixel as usize,
        render_pixel,
        |total, num_done| {
            let percentage = ((num_done as f32 / total as f32) * 100.).min(100.);
            let bar = get_bar(percentage);
            eprint!(
                "\r{} {:.2}%",
                bar,
                ((num_done as f32 / total as f32) * 100.).min(100.)
            );
        },
    );
    c.write_header();
    c.write_pixels();
    let elapsed = time.elapsed();
    eprint!("\nDone in {:.2}s\n", elapsed.as_secs_f32());
}

fn get_bar(percentage: f32) -> String {
    let to_color = (percentage / 2.).floor() as i32;
    let mut toret = String::from("[");

    for _ in 0..to_color {
        toret.push('#');
    }

    for _ in 0..(50 - to_color) {
        toret.push(' ');
    }
    toret.push(']');

    toret
}
