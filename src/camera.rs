use crate::{ray::Ray, vec3::*, utils::*};

pub struct Camera {
    origin: point3,
    lower_left_corner: point3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn new(lookfrom: point3, lookat: point3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Self {

        let theta = degrees_to_radians(vfov);
        let h = (theta/2.).tan();

        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);


        let focal_length = 1.;

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal/2. - vertical/2. -w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*s + self.vertical * t - self.origin)
    }
}
