use crate::objects::hittable::*;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: point3,
    pub dir: Vec3,
    pub tm: f32,
}
impl Ray {
    pub fn new(origin: point3, direction: Vec3, time: f32) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }
    pub fn origin(&self) -> point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f32 {
        self.tm
    }
    pub fn at(&self, t: f32) -> point3 {
        self.orig + self.dir * t
    }
}

pub fn ray_color(r: Ray, world: &impl Hittable, depth: i32) -> color {
    let mut rec = HitRecord::void();

    if depth <= 0 {
        return color::new();
    }

    if world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        let mut scattered = Ray::new(point3::new(), Vec3::new(), 0.);
        let mut attenuation = color::new();
        if rec
            .mat_ptr
            .scatter(r, rec , &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return color::from(0., 0., 0.);
    }
    let unit_direction = unit_vector(r.direction());

    let t = (unit_direction.y() + 1.) * 0.5;

    color::from(1., 1., 1.) * (1. - t) + color::from(0.5, 0.7, 1.) * t
}
