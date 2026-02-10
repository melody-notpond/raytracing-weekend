use std::io::Write;

use crate::color::Color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
    pub image_width: i32,
    pub image_height: i32,
    pub max_depth: i32,
    pub samples_per_pixel: i32,
    pub vfov: f32,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    center: Vec3,
    pixel_sample_scale: f32,
    aspect_ratio: f32,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    basis_u: Vec3,
    basis_v: Vec3,
    basis_w: Vec3,
}

impl Camera {
    pub fn new(image_width: i32, image_height: i32, look_from: Vec3,
        look_at: Vec3, up: Vec3) -> Self {
        Camera {
            image_width,
            image_height,
            max_depth: 10,
            samples_per_pixel: 10,
            vfov: 90.,
            aspect_ratio: 0.,
            look_from,
            look_at,
            up,
            center: Vec3::new(0., 0., 0.),
            pixel_sample_scale: 0.,
            pixel00_loc: Point3::new(0., 0., 0.),
            pixel_delta_u: Vec3::new(0., 0., 0.),
            pixel_delta_v: Vec3::new(0., 0., 0.),
            basis_u: Vec3::new(0., 0., 0.),
            basis_v: Vec3::new(0., 0., 0.),
            basis_w: Vec3::new(0., 0., 0.),
        }
    }

    pub fn render(&mut self, f: &mut impl Write, world: &impl Hittable) ->
        std::io::Result<()> {
        self.init();

        writeln!(f, "P3\n{} {}\n255\n", self.image_width,
            self.image_height)?;
        for j in 0..self.image_height {
            print!("\rscanlines remaining: {:3}", self.image_height - j);
            for i in 0..self.image_width {
                let mut color = Color::new(0., 0., 0.);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    color += self.ray_color(&ray, world, self.max_depth);
                }
                color *= self.pixel_sample_scale;
                color.write_color(f)?;
            }
        }

        println!("\rdone                       ");
        Ok(())
    }

    fn init(&mut self) {
        self.aspect_ratio = self.image_width as f32 / self.image_height as f32;
        self.pixel_sample_scale = 1. / self.samples_per_pixel as f32;

        self.center = self.look_from;
        
        // viewport
        let focal_length = (self.look_from - self.look_at).length();
        let h = (self.vfov.to_radians() / 2.).tan();
        let viewport_height = 2. * h * focal_length;
        let viewport_width = viewport_height * self.aspect_ratio;

        // orthonormal basis
        self.basis_w = (self.look_from - self.look_at).normalize();
        self.basis_u = self.up.cross(self.basis_w).normalize();
        self.basis_v = self.basis_w.cross(self.basis_u);

        // pixel location in world
        let viewport_u = viewport_width * self.basis_u;
        let viewport_v = viewport_height * -self.basis_v;
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;
        let viewport_upper_left = self.center - focal_length * self.basis_w
            - viewport_u / 2 - viewport_v / 2;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u +
            self.pixel_delta_v) / 2;
    }

    fn ray_color(&self, ray: &Ray, world: &impl Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0., 0., 0.)
        }

        let Some(hit) = world.hit(ray, 0.01, f32::INFINITY)
        else {
            let t = (ray.dir.y + 1.) / 2.;
            return (1. - t) * Color::new(1., 1., 1.) +
                t * Color::new(0.5, 0.7, 1.0)
        };

        let Some(scatter) = hit.material.scatter(ray, &hit)
        else {
            return Color::new(0., 0., 0.);
        };

        scatter.attenuation * self.ray_color(&scatter.scattered, world,
            depth - 1)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Vec3::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f32 + offset.x) * self.pixel_delta_u
            + (j as f32 + offset.y) * self.pixel_delta_v;
        Ray::new(self.center, pixel_sample - self.center)
    }
}
