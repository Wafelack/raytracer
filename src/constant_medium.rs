use crate::{
  aabb::*, material::material::*, objects::hittable::*, ray::*, texture::*, utils::*, vec3::*,
};
use std::sync::Arc;

pub struct ConstantMedium {
  boundary: Arc<dyn Hittable>,
  phase_function: Arc<dyn Material>,
  neg_inv_density: f32,
}

impl ConstantMedium {
  pub fn from_texture(b: Arc<dyn Hittable>, d: f32, a: Arc<dyn Texture>) -> Self {
    Self {
      boundary: b,
      neg_inv_density: -1. / d,
      phase_function: Arc::new(Isotropic::from_texture(a)),
    }
  }
  pub fn from_color(b: Arc<dyn Hittable>, d: f32, c: color) -> Self {
    Self {
      boundary: b,
      neg_inv_density: -1. / d,
      phase_function: Arc::new(Isotropic::from_color(c)),
    }
  }
}

impl Hittable for ConstantMedium {
  fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
    let enable_debug = false;
    let debugging = enable_debug && random_double(0., 1.) < 0.00001;

    let mut rec1 = HitRecord::void();
    let mut rec2 = HitRecord::void();

    if !self
      .boundary
      .hit(r, std::f32::NEG_INFINITY, std::f32::INFINITY, &mut rec1)
    {
      return false;
    }
    if !self
      .boundary
      .hit(r, rec1.t + 0.00001, std::f32::INFINITY, &mut rec2)
    {
      return false;
    }

    if debugging {
      eprintln!("\nt_min={}, t_max={}", rec1.t, rec2.t);
    }

    if rec1.t < t_min {
      rec1.t = t_min;
    }
    if rec2.t > t_max {
      rec2.t = t_max;
    }

    if rec1.t >= rec2.t {
      return false;
    }

    if rec1.t < 0. {
      rec1.t = 0.;
    }

    let ray_length = r.direction().len();
    let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
    let hit_distance = self.neg_inv_density * random_double(0., 1.).ln();

    if hit_distance > distance_inside_boundary {
      return false;
    }
    rec.t = rec1.t + hit_distance / ray_length;
    rec.p = r.at(rec.t);

    if debugging {
      eprintln!(
        "hit_distance = {}\n
                rec.t         = {}\n
                rec.p         = {:?}",
        hit_distance, rec.t, rec.p
      );
    }
    rec.normal = Vec3::from(1., 0., 0.);
    rec.front_face = true;
    rec.mat_ptr = &*self.phase_function;

    true
  }
  fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut Aabb) -> bool {
    self.boundary.bounding_box(time0, time1, output_box)
  }
}
