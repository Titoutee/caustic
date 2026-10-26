pub mod color;
pub mod vec3;

use vec3::{MulS, Vec3};

pub type Point3 = Vec3; // Useful alias for points, without the abstraction that a point really is a 3D vector...

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir.muls(t)
    }
}
