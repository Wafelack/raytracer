use crate::{aabb::*, material::material::*, ray::*, vec3::*};
use lazy_static::*;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub p: point3,
    pub normal: Vec3,
    pub mat_ptr: &'a dyn Material,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

lazy_static! {
    static ref DFL_LAMBERTIAN: Lambertian = Lambertian::from(color::new());
}

impl HitRecord<'_> {
    pub fn void() -> Self {
        Self {
            p: Vec3::new(),
            normal: Vec3::new(),
            mat_ptr: &*DFL_LAMBERTIAN,
            t: 0.,
            u: 0.,
            v: 0.,
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

pub trait Hittable: Send + Sync {
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool;
    fn bounding_box<'a>(&'a self, time0: f32, time1: f32, output_box: &mut Aabb) -> bool {
        true
    }
}
