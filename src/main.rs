use std::{io, io::Write};
use vector3d::*;

mod ray;
use ray::*;

fn write_color(pixel_color: Vector3d<f32>) {
    println!("{} {} {}", (pixel_color.x * 255.99) as i32, (pixel_color.y * 255.99) as i32, (pixel_color.z * 255.99) as i32);
}

fn main() {
    
    // Image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let color: Vector3d<f32> = Vector3d::new(i as f32 / ((IMAGE_WIDTH -1) as f32), j as f32 / ((IMAGE_HEIGHT -1) as f32), 0.25);
            write_color(color);
        }
    }
    eprint!("\n");io::stdout().flush().unwrap();
}
