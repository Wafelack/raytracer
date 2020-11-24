use crate::{objects::hittable::*, ray::*, vec3::*};

pub struct Sphere {
    pub center: point3,
    pub radius: f32,
}
impl Sphere {
    pub fn new(cen: point3, r: f32) -> Self {
        Self {
            center: cen,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
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
        rec.set_face_normal(r, outward_normal);

        true
    }
}
