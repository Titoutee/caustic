pub mod color;
pub mod vec3;

use vec3::{MulS, Vec3};

use crate::geometry::color::Color3;

pub type Point3 = Vec3; // Useful alias for points, without the abstraction that a point really is a 3D vector...

pub trait Object {
    fn hit(&self, ray: &Ray) -> bool;
}

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

    pub fn ray_color<T: Object>(&self, obj: T) -> Color3 {
        if obj.hit(self) {
            return Vec3(1., 0., 0.);
        }
        let unit_dir = self.dir.unit();
        let a = 0.5 * (unit_dir.0 + 1.0);
        Vec3(1., 1., 1.).muls(1.0 - a) + Vec3(0.5, 0.7, 1.).muls(a)
    }
}

pub mod objects {
    use super::Ray;
    use super::Vec3;
    use crate::geometry::{Object, Point3, vec3::Dot};
    #[derive(Debug)]
    pub struct Sphere {
        pub center: Point3,
        pub radius: f64,
    }

    impl Sphere {
        pub const fn new(center: Point3, radius: f64) -> Self {
            Sphere { center, radius }
        }
    }

    impl Object for Sphere {
        /// Detects ray intersection with the sphere based on quadratic evaluation
        fn hit(&self, ray: &Ray) -> bool {
            let oc = self.center - ray.origin;
            let a = ray.dir.dot(ray.dir);
            let b = -2.0 * ray.dir.dot(oc);
            let c = oc.dot(oc) - self.radius * self.radius;
            let discriminant = b * b - 4. * a * c;
            discriminant >= 0.
        }
    }

    pub const TEST_SPHERE: Sphere = Sphere::new(Vec3(0., 0., -1.), 0.5);
}
