use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl Hit {
    pub fn new(ray: &Ray, t: f32, normal: Vec3) -> Self {
        let front_face = ray.dir.dot(normal) < 0.;
        let normal = if front_face { normal } else { -normal };
        Self {
            point: ray.at(t),
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, start: f32, end: f32) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, start: f32, end: f32) -> Option<Hit> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_sq();
        let h = ray.dir.dot(oc);
        let c = oc.length_sq() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant.is_sign_negative() {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= start || root >= end {
            root = (h + sqrtd) / a;
            if root <= start || root >= end {
                return None;
            }
        }

        Some(Hit::new(ray, root, (ray.at(root) - self.center) / self.radius))
    }
}

impl<T> Hittable for Vec<T> where T: Hittable {
    fn hit(&self, ray: &Ray, start: f32, end: f32) -> Option<Hit> {
        let mut hit = None;
        let mut closest = end;

        for h in self.iter() {
            if let Some(hit_) = h.hit(ray, start, closest) {
                closest = hit_.t;
                hit = Some(hit_);
            }
        }

        hit
    }
}
