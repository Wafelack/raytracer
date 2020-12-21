use crate::{aabb::*, objects::hittable::*, objects::hittable_list::*, ray::*, utils::*};
use std::sync::Arc;

pub struct BvhNode {
  left: Arc<dyn Hittable>,
  right: Arc<dyn Hittable>,
  boxx: Aabb,
}

impl BvhNode {
  pub fn from(list: &HittableList, time0: f32, time1: f32) -> Self {
    Self::fromvec(&list.objects, 0, list.objects.len(), time0, time1)
  }
  pub fn fromvec(
    src_objects: &Vec<Arc<dyn Hittable>>,
    start: usize,
    end: usize,
    time0: f32,
    time1: f32,
  ) -> Self {
    let mut objects = src_objects;
    let axis = random_int(0, 2);

    let mut left = objects[start];
    let mut right = objects[start];

    let comparator = if axis == 0 {
      box_x_compare
    } else if axis == 1 {
      box_y_compare
    } else {
      box_z_compare
    };
    let object_span = end - start;

    if object_span == 1 {
    } else if object_span == 1 {
      if comparator(objects[start], objects[start + 1]) {
        left = objects[start];
        right = objects[start + 1];
      } else {
        right = objects[start];
        left = objects[start + 1];
      }
    } else {
    }
  }
}

impl Hittable for BvhNode {
  fn bounding_box<'a>(&'a self, time0: f32, time1: f32, output_box: &mut Aabb) -> bool {
    *output_box = self.boxx;
    true
  }
  fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
    if !self.boxx.hit(r, t_min, t_max) {
      return false;
    }

    let hit_left = self.left.hit(r, t_min, t_max, rec);
    let hit_right = self.right.hit(
      r,
      t_min,
      {
        if hit_left {
          rec.t
        } else {
          t_max
        }
      },
      rec,
    );

    hit_left || hit_right
  }
}
