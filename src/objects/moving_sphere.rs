use crate::{material::material::*, objects::hittable::*, ray::*, vec3::*};
use std::sync::Arc;

pub struct MovingSphere {
    pub center0: point3,
    pub center1: point3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        cen0: point3,
        cen1: point3,
        time0: f32,
        time1: f32,
        r: f32,
        m: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0,
            time1,
            radius: r,
            mat_ptr: m,
        }
    }
    pub fn center(&self, time: f32) -> point3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}
impl Hittable for MovingSphere {
    fn hit<'a>(&'a self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().len_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.len_squared() - self.radius * self.radius;
        let mut front_face = false;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = &*self.mat_ptr;

        true
    }
}
