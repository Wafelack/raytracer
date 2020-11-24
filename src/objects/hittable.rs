use crate::{ray::*, vec3::*};

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}
impl HitRecord {
    pub fn void() -> Self {
        Self {
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 0.,
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal.inv()
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
