use crate::{aabb::*, material::material::*, ray::*, utils::*, vec3::*};
use lazy_static::*;
use std::f32::INFINITY as inf;
use std::sync::Arc;

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
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        true
    }
    fn bounding_box<'a>(&'a self, time0: f32, time1: f32, output_box: &mut Aabb) -> bool {
        true
    }
}

pub struct Translate {
    offset: Vec3,
    ptr: Arc<dyn Hittable>,
}
impl Translate {
    pub fn from(p: Arc<dyn Hittable>, displacement: &Vec3) -> Self {
        Self {
            ptr: p,
            offset: *displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p = rec.p + self.offset;
        rec.set_face_normal(moved_r, rec.normal);
        true
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut Aabb) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }
        *output_box = Aabb::from(
            &(output_box.min() + self.offset),
            &(output_box.max() + self.offset),
        );

        true
    }
}

pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    has_box: bool,
    bbox: Aabb,
}

impl RotateY {
    pub fn from(p: Arc<dyn Hittable>, angle: f32) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let ptr = p.clone();
        let cos_theta = radians.cos();
        let mut bbox = Aabb::new();
        let has_box = p.bounding_box(0., 1., &mut bbox);

        let mut min = point3::from(inf, inf, inf);
        let mut max = point3::from(-inf, -inf, -inf);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.max().x() + (1. - i as f32) * bbox.min().x();
                    let y = i as f32 * bbox.max().y() + (1. - j as f32) * bbox.min().y();
                    let z = i as f32 * bbox.max().z() + (1. - k as f32) * bbox.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::from(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        bbox = Aabb::from(&min, &max);
        Self {
            ptr,
            sin_theta,
            cos_theta,
            has_box,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        self.has_box
    }
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;

        rec.set_face_normal(rotated_r, normal);

        true
    }
}
