use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction, }
    }

    pub fn origin(&self) -> Vec3 { self.origin }
    pub fn direction(&self) -> Vec3 { self.direction }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
