use crate::{perlin::*, vec3::*};
use std::sync::Arc;

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

pub struct CheckerTexture {
  odd: Arc<dyn Texture>,
  even: Arc<dyn Texture>,
}

impl CheckerTexture {
  pub fn from_texture(_even: Arc<dyn Texture>, _odd: Arc<dyn Texture>) -> Self {
    Self {
      odd: _odd,
      even: _even,
    }
  }
  pub fn from_colors(c1: color, c2: color) -> Self {
    Self {
      even: Arc::new(SolidColor::from(c1)),
      odd: Arc::new(SolidColor::from(c2)),
    }
  }
}

impl Texture for CheckerTexture {
  fn value(&self, u: f32, v: f32, p: &point3) -> color {
    let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
    if sines < 0. {
      return self.odd.value(u, v, p);
    } else {
      return self.even.value(u, v, p);
    }
  }
}
pub struct NoiseTexture {
  noise: Perlin,
  scale: f32,
}
impl NoiseTexture {
  pub fn new() -> Self {
    Self {
      noise: Perlin::new(),
      scale: 2.,
    }
  }
  pub fn from(sc: f32) -> Self {
    Self {
      noise: Perlin::new(),
      scale: sc,
    }
  }
}

impl Texture for NoiseTexture {
  fn value(&self, u: f32, v: f32, p: &point3) -> color {
    color::from(1., 1., 1.) * self.noise.noise(&(*p * self.scale))
  }
}
