use std::ops::*;

use rand::distr::Distribution;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z}
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        const EPSILON: f32 = 1e-8;
        self.x.abs() < EPSILON && self.y.abs() < EPSILON &&
            self.z.abs() < EPSILON
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2. * normal.dot(self) * normal
    }

    pub fn refract(self, normal: Vec3, index_ratio: f32) -> Vec3 {
        let cos_theta = self.dot(-normal).min(1.);
        let perp = index_ratio * (self + cos_theta * normal);
        let para = -(1. - perp.length_sq()).abs().sqrt() * normal;
        perp + para
    }

    pub fn random() -> Vec3 {
        let mut guard = crate::UNIFORM.lock().unwrap();
        let uniform = guard.get_mut().unwrap();
        Vec3 {
            x: uniform.sample(&mut rand::rng()),
            y: uniform.sample(&mut rand::rng()),
            z: uniform.sample(&mut rand::rng()),
        }
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        let mut guard = crate::UNIFORM.lock().unwrap();
        let uniform = guard.get_mut().unwrap();
        Vec3 {
            x: uniform.sample(&mut rand::rng()) * (max - min) + min,
            y: uniform.sample(&mut rand::rng()) * (max - min) + min,
            z: uniform.sample(&mut rand::rng()) * (max - min) + min,
        }
    }

    pub fn random_unit() -> Vec3 {
        let mut guard = crate::UNIFORM.lock().unwrap();
        let uniform = guard.get_mut().unwrap();
        loop {
            let p = Vec3 {
                x: uniform.sample(&mut rand::rng()) * 2. - 1.,
                y: uniform.sample(&mut rand::rng()) * 2. - 1.,
                z: uniform.sample(&mut rand::rng()) * 2. - 1.,
            };
            let len_sq = p.length_sq();
            if len_sq.is_finite() && len_sq <= 1. {
                return p / len_sq.sqrt();
            }
        }
    }

    pub fn random_hemisphere(normal: Vec3) -> Vec3 {
        let unit = Self::random_unit();
        if unit.dot(normal) >= 0. {
            return unit;
        }
        -unit
    }

    pub fn sample_square() -> Vec3 {
        let mut guard = crate::UNIFORM.lock().unwrap();
        let uniform = guard.get_mut().unwrap();
        Vec3::new(uniform.sample(&mut rand::rng()) - 0.5,
            uniform.sample(&mut rand::rng()) - 0.5, 0.)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec3 {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
        }
    }
}

impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self as f32,
            y: rhs.y * self as f32,
            z: rhs.z * self as f32,
        }
    }
}

impl Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        Vec3 {
            x: self.x / rhs as f32,
            y: self.y / rhs as f32,
            z: self.z / rhs as f32,
        }
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl MulAssign<i32> for Vec3 {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl DivAssign<i32> for Vec3 {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}
