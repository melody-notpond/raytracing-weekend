use std::fs::File;
use std::rc::Rc;

use raytracing::camera::Camera;
use raytracing::color::Color;
use raytracing::hittable::Sphere;
use raytracing::material::{Lambertian, Metal};
use raytracing::vec3::*;

fn main() -> std::io::Result<()> {
    raytracing::init_uniform();
    let mut file = File::create("image.ppm")?;
    let mut camera = Camera::new(800, 600, Point3::new(0., 0., 0.));

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let mat_centre = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.8));
    let world = vec![
        Sphere::new(Point3::new(0., -100.5, -1.), -100., mat_ground),
        Sphere::new(Point3::new(0., 0., -1.2), 0.5, mat_centre),
        Sphere::new(Point3::new(-1., 0., -1.), 0.5, mat_left),
        Sphere::new(Point3::new(1., 0., -1.), 0.5, mat_right),
    ];

    camera.render(&mut file, &world)
}
