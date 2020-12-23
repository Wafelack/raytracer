use crate::{utils::*, vec3::*};

pub struct Perlin {
  point_count: usize,
  ranvec: Vec<Vec3>,
  perm_x: Vec<i32>,
  perm_y: Vec<i32>,
  perm_z: Vec<i32>,
}

impl Perlin {
  pub fn new() -> Self {
    let point_count = 256usize;
    let mut ranvec: Vec<Vec3> = Vec::with_capacity(point_count);

    for _ in 0..point_count {
      ranvec.push(unit_vector(Vec3::random(-1., 1.)));
    }
    let perm_x = Perlin::generate_perm(point_count);
    let perm_y = Perlin::generate_perm(point_count);
    let perm_z = Perlin::generate_perm(point_count);

    Self {
      point_count,
      ranvec,
      perm_x,
      perm_y,
      perm_z,
    }
  }
  pub fn noise(&self, p: &point3) -> f32 {
    let mut u = p.x() - p.x().floor();
    let mut v = p.y() - p.y().floor();
    let mut w = p.z() - p.z().floor();

    u = u * u * (3. - 2. * u);
    v = v * v * (3. - 2. * v);
    w = w * w * (3. - 2. * w);

    let i = p.x().floor() as i32;
    let j = p.y().floor() as i32;
    let k = p.z().floor() as i32;

    let mut c = [[[Vec3::new(); 2]; 2]; 2];

    for di in 0..2 {
      for dj in 0..2 {
        for dk in 0..2 {
          c[di][dj][dk] = self.ranvec[(self.perm_x[(i + di as i32) as usize & 255]
            ^ self.perm_y[(j + dj as i32) as usize & 255]
            ^ self.perm_z[(k + dk as i32) as usize & 255])
            as usize];
        }
      }
    }

    Self::perlin_interp(c, u, v, w)
  }
  fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);

    let mut accum = 0.;
    for i in 0..2 {
      for j in 0..2 {
        for k in 0..2 {
          let weight_v = Vec3::from(u - i as f32, v - j as f32, w - k as f32);
          accum += (i as f32 * uu as f32 + (1. - i as f32) * (1. - uu))
            * (j as f32 * vv as f32 + (1. - j as f32) * (1. - vv))
            * (k as f32 * ww as f32 + (1. - k as f32) * (1. - ww))
            * dot(c[i][j][k], weight_v);
        }
      }
    }
    accum
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
