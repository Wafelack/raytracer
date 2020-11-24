use crate::{ray::Ray, vec3::*, utils::*};

pub struct Camera {
    origin: point3,
    lower_left_corner: point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}
impl Camera {
    pub fn new(lookfrom: point3, lookat: point3, vup: Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {

        let theta = degrees_to_radians(vfov);
        let h = (theta/2.).tan();

        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);


        let focal_length = 1.;

        let origin = lookfrom;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;
        let lower_left_corner = origin - horizontal/2. - vertical/2. - w * focus_dist;

        let lens_radius = aperture / 2.;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {

        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal*s + self.vertical * t - self.origin - offset)
    }
}
