use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir: dir.normalize() }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}
