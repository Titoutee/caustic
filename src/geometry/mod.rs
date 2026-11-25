pub mod color;
pub mod vec3;

use crate::geometry::{color::Color3, vec3::Dot};
use std::f64::consts::PI;
use std::sync::Arc;
use vec3::{MulS, Vec3}; // Or use Box if single-threaded

pub type Point3 = Vec3; // Useful alias for points, without the abstraction that a point really is a 3D vector...

#[inline]
pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.
}

#[derive(Default, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir.dot(*outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            !*outward_normal
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn from_object(obj: Arc<dyn Hittable>) -> Self {
        HittableList { objects: vec![obj] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Arc::new(obj));
    }

    pub fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_tmax;

        for obj in &self.objects {
            if let Some(rec_) = obj.hit(ray, ray_tmin, closest_so_far) {
                rec = Some(rec_);
                closest_so_far = rec.unwrap().t;
            }
        }

        rec
    }
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

    pub fn ray_color(&self, world: &HittableList) -> Color3 {
        let hit = world.hit(self, 0., f64::MAX);
        if let Some(hit) = hit {
            return (hit.normal + Vec3(1., 1., 1.)).muls(0.5);
        }
        let unit_dir = self.dir.unit();
        let a = 0.5 * (unit_dir.0 + 1.0);
        Vec3(1., 1., 1.).muls(1.0 - a) + Vec3(0.5, 0.7, 1.).muls(a)
    }
}

pub mod objects {
    use super::Ray;
    use super::Vec3;
    use crate::geometry::HitRecord;
    use crate::geometry::vec3::MulS;
    use crate::geometry::{Hittable, Point3, vec3::Dot};
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

    impl Hittable for Sphere {
        /// Detects ray intersection with the sphere based on quadratic evaluation
        /// It contains simplification of the basis quadratic equation given that member b = -2h where h = d dot (C-P), which is exactly what's computed here
        fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
            let oc = self.center - ray.origin;
            let a = ray.dir.length_squared();
            let h = ray.dir.dot(oc);
            let c = oc.dot(oc) - self.radius * self.radius;
            let discriminant = h * h - a * c;

            if discriminant < 0. {
                return None;
            }

            let sqrtd = discriminant.sqrt();

            let mut root = (h - sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                root = (h + sqrtd) / a; // We test the second root
                if root <= ray_tmin || root >= ray_tmax {
                    return None;
                }
            }

            let t = root;
            let p = ray.at(t);
            let outward_normal = (p - self.center).muls(1. / self.radius);

            let mut rec = HitRecord {
                p,
                normal: outward_normal,
                t,
                front_face: true,
            };

            // And mutates the record according to the orientation of the outwarding normal
            rec.set_face_normal(ray, &outward_normal);

            Some(rec)
        }
    }

    pub const TEST_SPHERE: Sphere = Sphere::new(Vec3(0., 0., -1.), 0.5);
}
