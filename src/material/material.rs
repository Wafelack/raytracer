use crate::{ray::*, vec3::*,utils::*,  objects::hittable::HitRecord};

pub trait Material: Send + Sync{
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut color, scattered: &mut Ray) -> bool;
}
#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: color,
}
impl Lambertian {
    pub fn from(a: color) -> Self {
        Self {albedo: a}
    }
}
impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + color::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo;
        true

    }
}
#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: color,
    pub fuzz: f32,
}
impl Metal {
    pub fn from(a: color, f: f32) -> Self {
        if f  < 1. {
            return Self { albedo:a , fuzz: f};
        } else {
            return Self { albedo: a, fuzz: 1.};
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut color, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz, r_in.time());
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ir: f32
}
impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self { ir: index_of_refraction}
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut color, scattered: &mut Ray) -> bool {
        *attenuation = color::from(1.,1.,1.);
        let refraction_ratio = if rec.front_face {
            1./self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(r_in.direction());

        let cos_theta = dot(unit_direction.inv(), rec.normal).min(1.);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();
        
        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let mut direction = Vec3::new();

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double(0., 1.){
            direction = reflect(unit_direction, rec.normal);
        } else {
            direction = refract(unit_direction, rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.p, direction, r_in.time());
       
        true
    }
}
fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.-ref_idx) / (1.+ref_idx);
    r0 = r0*r0;
    r0 + (1.-r0)*(1.-cosine).powi(5)
}
