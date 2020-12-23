use crate::{aabb::*, objects::hittable::*, objects::hittable_list::*, ray::*, utils::*};
use std::ops::Deref;
use std::sync::Arc;

pub struct BvhNode {
  left: Arc<dyn Hittable>,
  right: Arc<dyn Hittable>,
  boxx: Aabb,
}

impl BvhNode {
  pub fn from(list: &mut HittableList, time0: f32, time1: f32) -> Self {
    let len = &mut list.objects.len();
    Self::fromvec(&mut list.objects, 0, *len, time0, time1)
  }
  pub fn fromvec(
    src_objects: &mut Vec<Arc<dyn Hittable>>,
    start: usize,
    end: usize,
    time0: f32,
    time1: f32,
  ) -> Self {
    let objects = src_objects;
    let axis = random_int(0, 2);

    let mut left: Arc<dyn Hittable> = objects[start].clone();
    let mut right: Arc<dyn Hittable> = objects[start].clone();

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
      if comparator(objects[start].deref(), objects[start + 1].deref()) {
        left = objects[start].clone();
        right = objects[start + 1].clone();
      } else {
        right = objects[start].clone();
        left = objects[start + 1].clone();
      }
    } else {
      &objects[start..end].sort_unstable_by(|a, b| {
        if comparator(&**a, &**b) {
          std::cmp::Ordering::Greater
        } else {
          std::cmp::Ordering::Less
        }
      });

      let mid = start + object_span / 2;
      left = Arc::new(BvhNode::fromvec(objects, start, mid, time0, time1)) as Arc<dyn Hittable>;
    }
    let mut box_left = Aabb::new();
    let mut box_right = Aabb::new();

    if !left.bounding_box(time0, time1, &mut box_left)
      || !right.bounding_box(time0, time1, &mut box_right)
    {
      eprintln!("No bounding box in bvh_node constructor");
    }

    let boxx = surrounding_box(&box_left, &box_right);

    Self {
      left: left.clone(),
      right: right.clone(),
      boxx,
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

fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> bool {
  let mut box_a = Aabb::new();
  let mut box_b = Aabb::new();

  if !a.bounding_box(0., 0., &mut box_a) || !b.bounding_box(0., 0., &mut box_b) {
    eprintln!("No bounding box in BvhNode constructor");
  }
  box_a.min().e[axis] < box_b.min().e[axis]
}

fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
  box_compare(a, b, 0)
}
fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
  box_compare(a, b, 1)
}
fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
  box_compare(a, b, 2)
}
