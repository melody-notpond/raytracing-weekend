use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

fn linear_to_gamma(linear: f32) -> f32 {
    if linear <= 0. {
        0.
    } else {
        linear.sqrt()
    }
}

impl Color {
    pub fn write_color(self, f: &mut impl Write) ->
        std::io::Result<()> {
        let r = linear_to_gamma(self.x.clamp(0., 1.));
        let g = linear_to_gamma(self.y.clamp(0., 1.));
        let b = linear_to_gamma(self.z.clamp(0., 1.));
        let ir = (r * 255.999) as i32;
        let ig = (g * 255.999) as i32;
        let ib = (b * 255.999) as i32;
        writeln!(f, "{ir} {ig} {ib}")
    }
}
