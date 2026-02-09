use crate::{color::Color, hittable::Hit, ray::Ray, vec3::Vec3};

pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut scatter_dir = hit.normal + Vec3::random_unit();
        if scatter_dir.near_zero() {
            scatter_dir = hit.normal;
        }

        Some(Scatter {
            scattered: Ray::new(hit.point, scatter_dir),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = ray.dir.reflect(hit.normal).normalize() +
            self.fuzz * Vec3::random_unit();
        if reflected.dot(hit.normal) > 0. {
            Some(Scatter {
                scattered: Ray::new(hit.point, reflected),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
