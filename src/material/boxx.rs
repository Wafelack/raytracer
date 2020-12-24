use crate::{
  aarect::*, material::material::*, objects::hittable::*, objects::hittable_list::*, ray::*,
  vec3::*,
};
use std::sync::Arc;

pub struct Boxx {
  box_min: point3,
  box_max: point3,
  sides: HittableList,
}

impl Boxx {
  pub fn new() -> Self {
    Self {
      box_min: point3::new(),
      box_max: point3::new(),
      sides: HittableList::new(),
    }
  }
  pub fn from(p0: &point3, p1: &point3, ptr: Arc<dyn Material>) -> Self {
    let box_min = p0;
    let box_max = p1;

    let mut sides = HittableList::new();

    sides.add(Arc::new(XyRect::from(
      p0.x(),
      p1.x(),
      p0.y(),
      p1.y(),
      p1.z(),
      ptr.clone(),
    )));

    sides.add(Arc::new(XyRect::from(
      p0.x(),
      p1.x(),
      p0.y(),
      p1.y(),
      p0.z(),
      ptr.clone(),
    )));

    sides.add(Arc::new(XzRect::from(
      p0.x(),
      p1.x(),
      p0.z(),
      p1.z(),
      p1.y(),
      ptr.clone(),
    )));

    sides.add(Arc::new(XzRect::from(
      p0.x(),
      p1.x(),
      p0.z(),
      p1.z(),
      p0.y(),
      ptr.clone(),
    )));

    sides.add(Arc::new(YzRect::from(
      p0.y(),
      p1.y(),
      p0.z(),
      p1.z(),
      p1.x(),
      ptr.clone(),
    )));

    sides.add(Arc::new(YzRect::from(
      p0.y(),
      p1.y(),
      p0.z(),
      p1.z(),
      p0.x(),
      ptr.clone(),
    )));

    Self {
      box_min: *box_min,
      box_max: *box_max,
      sides,
    }
  }
}

impl Hittable for Boxx {
  fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
    self.sides.hit(r, t_min, t_max, rec)
  }
}
