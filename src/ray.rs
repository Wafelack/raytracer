use crate::vec3::*;

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
