use crate::{ray::*, vec3::*, objects::hittable::HitRecord};

pub trait Material {
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

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true

    }
}
#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: color,
}
impl Metal {
    pub fn from(a: color) -> Self {
        Self {albedo: a}
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut color, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.
    }
}