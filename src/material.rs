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

pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let index = if hit.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let dir = ray.dir.normalize();
        let cos_theta = dir.dot(-hit.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_reflect = index * sin_theta > 1.;

        // schlicks approximation for reflectance
        let r0 = ((1. - index) / (1. + index)).powi(2);
        let reflectance = r0 + (1. - r0) * (1. - cos_theta).powi(5);

        let scattered = if cannot_reflect || reflectance > crate::random_f32() {
            dir.reflect(hit.normal)
        } else {
            dir.refract(hit.normal, index)
        };

        Some(Scatter {
            scattered: Ray::new(hit.point, scattered),
            attenuation: Color::new(1., 1., 1.),
        })
    }
}
