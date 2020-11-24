use crate::vec3::*;
use crate::objects::hittable::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: point3,
    pub dir: Vec3,
}
impl Ray {
    pub fn new(origin: point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    pub fn origin(&self) -> point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
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
        let target = rec.p + rec.normal + color::random_in_unit_sphere();
        return ray_color(Ray::new(rec.p, target - rec.p), world, depth-1) * 0.5;
    }
    let unit_direction = unit_vector(r.direction());

    let t = (unit_direction.y() + 1.) * 0.5;

    color::from(1., 1., 1.) * (1. - t) + color::from(0.5, 0.7, 1.) * t
}