use crate::{ray::Ray, vec3::*};

pub struct Camera {
    origin: point3,
    lower_left_corner: point3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin = point3::new();
        let horizontal = Vec3::from(viewport_width, 0., 0.);
        let vertical = Vec3::from(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::from(0., 0., focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
