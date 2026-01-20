use std::fs::File;

use raytracing::camera::Camera;
use raytracing::hittable::Sphere;
use raytracing::vec3::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("image.ppm")?;
    let mut camera = Camera::new(800, 600, Point3::new(0., 0., 0.));
    let world = vec![
        Sphere::new(Point3::new(0., 0., -1.), 0.5),
        Sphere::new(Point3::new(0., -100.5, -1.), -100.),
    ];

    camera.render(&mut file, &world)
}
