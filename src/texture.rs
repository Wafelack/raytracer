use crate::vec3::*;

pub trait Texture: Send + Sync {
  fn value(&self, u: f32, v: f32, p: &point3) -> color;
}
#[derive(Clone)]
pub struct SolidColor {
  color_value: color,
}
impl SolidColor {
  pub fn new() -> Self {
    Self {
      color_value: color::new(),
    }
  }
  pub fn from(c: color) -> Self {
    Self { color_value: c }
  }
  pub fn fromrgb(r: f32, g: f32, b: f32) -> Self {
    Self {
      color_value: color::from(r, g, b),
    }
  }
}

impl Texture for SolidColor {
  fn value(&self, u: f32, v: f32, p: &point3) -> color {
    self.color_value
  }
}
