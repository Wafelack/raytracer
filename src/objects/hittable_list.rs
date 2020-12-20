use std::sync::Arc;

use crate::{objects::hittable::*, ray::*};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        while self.objects.len() > 0 {
            self.objects.pop();
        }
    }
}

impl Hittable for HittableList {
    fn hit<'a>(&'a self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let mut temp_rec = HitRecord::void();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
