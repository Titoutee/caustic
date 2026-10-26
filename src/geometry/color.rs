use super::vec3;
use vec3::Vec3;

pub type Color3 = Vec3;

impl Color3 {
    pub fn write(&self) {
        let (r, g, b) = self.coords();

        let rbyte = (255.99 * r) as usize;
        let gbyte = (255.99 * g) as usize;
        let bbyte = (255.99 * b) as usize;
    }
}
