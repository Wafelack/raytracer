use crate::{aabb::*, material::material::Material, objects::hittable::*, ray::*, vec3::*};
use std::f32::consts::PI;
use std::sync::Arc;

pub struct Sphere {
    pub center: point3,
    pub radius: f32,
    pub mat_ptr: Arc<dyn Material>,
}
impl Sphere {
    pub fn new(cen: point3, r: f32, m: Arc<dyn Material>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }
}
pub fn get_sphere_uv(p: &point3, u: &mut f32, v: &mut f32) {
    let theta = p.inv().y().acos();
    let phi = p.inv().z().atan2(p.x()) + PI;

    *u = phi / (2. * PI);
    *v = theta / PI;
}

impl Hittable for Sphere {
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let oc = r.origin() - self.center;
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(*r, outward_normal);
        get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = &*self.mat_ptr;

        true
    }

    fn bounding_box<'a>(&'a self, time0: f32, time1: f32, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::from(
            &(self.center - Vec3::from(self.radius, self.radius, self.radius)),
            &(self.center + Vec3::from(self.radius, self.radius, self.radius)),
        );
        true
    }
}
