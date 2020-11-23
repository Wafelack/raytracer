use vector3d::*;

struct Ray {
    pub orig: Vector3d<f32>,
    pub dir: Vector3d<f32>
}
impl Ray {
    pub fn new(origin: Vector3d<f32>, direction: Vector3d<f32>) -> Self {
        Ray { orig: origin, dir: direction}
    }
    pub fn origin(&self) -> Vector3d<f32> {
        self.orig
    }
    pub fn direction(&self) -> Vector3d<f32> {
        self.dir
    }
    pub fn at(&self, t: f32) -> Vector3d<f32> {
        self.orig + self.dir * t
    }
}