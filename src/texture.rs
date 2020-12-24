use crate::{perlin::*, utils::*, vec3::*};
use stb_image::{image::load, image::LoadResult::*};
use std::path::Path;
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
    color::from(1., 1., 1.) * 0.5 * (1. + (self.scale * p.z() + 10. * self.noise.turb(&p, 7)).sin())
  }
}

pub struct ImageTexture {
  pub bytes_per_pixel: usize,
  data: Vec<u8>,
  width: usize,
  height: usize,
  bytes_per_scanline: usize,
}

impl ImageTexture {
  pub fn new() -> Self {
    Self {
      data: vec![],
      bytes_per_pixel: 3,
      width: 0,
      height: 0,
      bytes_per_scanline: 0,
    }
  }
  pub fn from(filename: &str) -> Self {
    let bytes_per_pixel = 3;

    if !Path::new(filename).exists() {
      eprintln!("ERROR: file `{}` doesn't exist !", filename);
      return Self {
        bytes_per_pixel,
        data: vec![],
        width: 0,
        height: 0,
        bytes_per_scanline: 0,
      };
    }

    let (width, height, data) = match load(filename) {
      ImageU8(v) => (v.width, v.height, v.data),
      _ => {
        eprintln!("ERROR: Could not load texture image file {}", filename);
        (0, 0, vec![])
      }
    };
    let bytes_per_scanline = bytes_per_pixel * width;
    Self {
      bytes_per_pixel,
      data,
      width,
      height,
      bytes_per_scanline,
    }
  }
}

impl Texture for ImageTexture {
  fn value(&self, u: f32, v: f32, p: &point3) -> color {
    if self.data.len() == 0 {
      return color::new();
    }

    let uu = clamp(u, 0., 1.);
    let vv = 1. - clamp(v, 0., 1.);

    let mut i = (uu * self.width as f32) as usize;
    let mut j = (vv * self.height as f32) as usize;

    if i >= self.width {
      i = self.width - 1;
    }
    if j >= self.height {
      j = self.height - 1;
    }

    let color_scale = 1. / 255.;
    let offset = j * self.bytes_per_scanline + i * self.bytes_per_pixel;
    let pixel = &self.data[offset..];

    color::from(
      color_scale * pixel[0] as f32,
      color_scale * pixel[1] as f32,
      color_scale * pixel[2] as f32,
    )
  }
}
