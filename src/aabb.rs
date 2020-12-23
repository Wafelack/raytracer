use crate::*;

#[derive(Copy, Clone, Default)]
pub struct Aabb {
  minimum: point3,
  maximum: point3,
}
pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
  let small = point3::from(
    min(box0.min().x(), box1.min().x()),
    min(box0.min().y(), box1.min().y()),
    min(box0.min().z(), box1.min().z()),
  );

  let big = point3::from(
    max(box0.min().x(), box1.min().x()),
    max(box0.min().y(), box1.min().y()),
    max(box0.min().z(), box1.min().z()),
  );

  Aabb::from(&small, &big)
}
impl Aabb {
  pub fn new() -> Self {
    Self {
      minimum: point3::new(),
      maximum: point3::new(),
    }
  }
  pub fn from(a: &point3, b: &point3) -> Self {
    Self {
      minimum: *a,
      maximum: *b,
    }
  }
  pub fn min(&self) -> point3 {
    self.minimum
  }
  pub fn max(&self) -> point3 {
    self.maximum
  }
  pub fn hit(&self, r: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
    for i in 0..3 {
      let inv_d = 1.0 / r.direction()[i];

      let mut t0 = (self.min()[i] - r.origin()[i]) * inv_d;
      let mut t1 = (self.max()[i] - r.origin()[i]) * inv_d;

      if inv_d < 0.0 {
        let prev_t0 = t0;
        t0 = t1;
        t1 = prev_t0;
      }
      t_min = if t0 > t_min { t0 } else { t_min };
      t_max = if t1 < t_max { t1 } else { t_max };

      if t_max <= t_min {
        return false;
      }
    }
    true
  }
}

fn min(a: f32, b: f32) -> f32 {
  if a == std::f32::NAN {
    return b;
  } else if b == std::f32::NAN {
    return a;
  }
  if a < b {
    return a;
  } else {
    return b;
  }
}
fn max(a: f32, b: f32) -> f32 {
  if a == std::f32::NAN {
    return b;
  } else if b == std::f32::NAN {
    return a;
  }
  if a > b {
    return a;
  } else {
    return b;
  }
}
