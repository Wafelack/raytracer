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

pub fn ray_color(r: Ray, background: &color, world: &impl Hittable, depth: i32) -> color {
    let mut rec = HitRecord::void();

    if depth <= 0 {
        return color::new();
    }

    if !world.hit(&r, 0.001, f32::INFINITY, &mut rec) {
        return *background;
    }
    let mut scattered = Ray::new(point3::new(), Vec3::new(), 0.);
    let mut attenuation = color::new();
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);

    if !rec
        .mat_ptr
        .scatter(r, rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }

    emitted + attenuation * ray_color(scattered, background, world, depth - 1)
}
