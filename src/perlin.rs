use crate::{utils::*, vec3::*};

pub struct Perlin {
  point_count: usize,
  ranfloat: Vec<f32>,
  perm_x: Vec<i32>,
  perm_y: Vec<i32>,
  perm_z: Vec<i32>,
}

impl Perlin {
  pub fn new() -> Self {
    let point_count = 256usize;
    let mut ranfloat: Vec<f32> = Vec::with_capacity(point_count);

    for _ in 0..point_count {
      ranfloat.push(random_double(0., 1.));
    }
    let perm_x = Perlin::generate_perm(point_count);
    let perm_y = Perlin::generate_perm(point_count);
    let perm_z = Perlin::generate_perm(point_count);

    Self {
      point_count,
      ranfloat,
      perm_x,
      perm_y,
      perm_z,
    }
  }
  pub fn noise(&self, p: &point3) -> f32 {
    let i = (4. * p.x()) as i32 & 255;
    let j = (4. * p.y()) as i32 & 255;
    let k = (4. * p.z()) as i32 & 255;
    self.ranfloat
      [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
  }
  fn generate_perm(point_count: usize) -> Vec<i32> {
    let mut p = Vec::with_capacity(point_count);
    for i in 0..point_count {
      p.push(i as i32);
    }
    Perlin::permute(&mut p, point_count);
    p
  }
  fn permute(p: &mut Vec<i32>, n: usize) {
    for i in (n - 1..0).rev() {
      let target = random_int(0, i as i32);
      let tmp = p[i];
      p[i] = p[target as usize];
      p[target as usize] = tmp;
    }
  }
}
