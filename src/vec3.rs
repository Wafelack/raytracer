use std::ops;
use crate::utils::random_double;
use std::cmp::min;

#[derive(Copy, Clone , Default)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Into<(f32 , f32 , f32)> for Vec3{
    fn into(self) -> (f32 , f32 ,f32){
        (self.x() , self.y() , self.z())
    }
}

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.,0.,0.]}
    }
    pub fn from(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: [e0, e1, e2]}
    }

    pub fn random(min: f32, max: f32) -> Self {
        Vec3::from(random_double(min, max), random_double(min, max), random_double(min, max))
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random(-1., 1.);
            if p.len_squared() >= 1. {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Self {
        unit_vector(Self::random_in_unit_sphere())
    }
    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if dot(in_unit_sphere, normal) > 0. {
            return in_unit_sphere;
        } else {
            return in_unit_sphere.inv();
        }
    }
    pub fn random_in_unit_disk() -> Self{
        loop {
            let p = Self::from(random_double(-1., 1.),random_double(-1., 1.), 0.);
            if p.len_squared() >= 1. { 
                continue; 
            }
            return p;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s:f32 = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    /// Used to get the inverted values of the vec
    pub fn inv(&self) -> Self {
        Vec3::from(-self.x(), -self.y(), -self.z())
    }
    pub fn get_at(&self, i: usize) -> f32 {
        self.e[i]
    }
    pub fn add(&mut self, v: Vec3) {
        for i in 0..self.e.len() {
            self.e[i] += v.e[i];
        }
    }
    pub fn mul(&mut self, t: f32) {
        for i in 0..self.e.len() {
            self.e[i] *= t;
        }
    }
    pub fn div(&mut self, t: f32) {
        self.mul(1./t);
    }
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }
    pub fn len_squared(&self) -> f32 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from(self.e[0] + other.e[0], self.e[1] + other.e[1],self.e[2] + other.e[2])
    }

}
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from(self.e[0] - other.e[0], self.e[1] - other.e[1],self.e[2] - other.e[2])
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, v: Self) -> Self {
        Vec3::from(self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2])
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, t: f32) -> Self {
        Self::from(t*self.e[0], t*self.e[1], t*self.e[2])
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self {
        self * (1./t)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::from(u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0])
}
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.len()
}
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v -  n *(2. * dot(v,n))
}
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat:f32) -> Vec3 {
    let cos_theta = dot(uv.inv(), n).min(1.);
    let r_out_perp: Vec3 =  (uv + n*cos_theta) * etai_over_etat;
    let r_out_parallel: Vec3 = n * -((1.0 - r_out_perp.len_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}  

pub type point3 = Vec3; 
pub type color = Vec3;
