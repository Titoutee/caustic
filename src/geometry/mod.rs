pub mod color;
pub mod vec3;

use vec3::{MulS, Vec3};

use crate::geometry::color::Color3;

pub type Point3 = Vec3; // Useful alias for points, without the abstraction that a point really is a 3D vector...

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir.muls(t)
    }

    pub fn ray_color(&self) -> Color3 {
        let unit_dir = self.dir.unit();
        let a = 0.5 * (unit_dir.0 + 1.0);
        Vec3(1., 1., 1.).muls(1.0 - a) + Vec3(0.5, 0.7, 1.).muls(a)
    }
}
