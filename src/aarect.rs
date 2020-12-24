use crate::{aabb::*, material::material::*, objects::hittable::*, ray::*, vec3::*};
use std::sync::Arc;

pub struct XyRect {
  mp: Arc<dyn Material>,
  x0: f32,
  x1: f32,
  y0: f32,
  y1: f32,
  k: f32,
}

impl XyRect {
  pub fn new() -> Self {
    Self {
      mp: Arc::new(Lambertian::from(color::new())),
      x0: 0.,
      x1: 0.,
      y0: 0.,
      y1: 0.,
      k: 0.,
    }
  }
  pub fn from(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mat: Arc<dyn Material>) -> Self {
    Self {
      x0,
      x1,
      y0,
      y1,
      k,
      mp: mat,
    }
  }
}

impl Hittable for XyRect {
  fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut Aabb) -> bool {
    *output_box = Aabb::from(
      &point3::from(self.x0, self.y0, self.k - 0.0001),
      &point3::from(self.x1, self.y1, self.k + 0.0001),
    );
    true
  }
  fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
    let t = (self.k - r.origin().z()) / r.direction().z();
    if t < t_min || t > t_max {
      return false;
    }
    let x = r.origin().x() + t * r.direction().x();
    let y = r.origin().y() + t * r.direction().y();
    if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
      return false;
    }

    rec.u = (x - self.x0) / (self.x1 - self.x0);
    rec.v = (x - self.y0) / (self.y1 - self.y0);

    rec.t = t;

    let outward_normal = Vec3::from(0., 0., 1.);
    rec.set_face_normal(*r, outward_normal);
    rec.mat_ptr = &*self.mp;
    rec.p = r.at(t);

    true
  }
}
