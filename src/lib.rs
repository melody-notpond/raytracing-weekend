use std::cell::OnceCell;
use std::sync::Mutex;

use rand::distr::Uniform;

static UNIFORM: Mutex<OnceCell<Uniform<f32>>> = Mutex::new(OnceCell::new());

pub fn init_uniform() {
    let uniform = UNIFORM.lock().unwrap();
    uniform.set(Uniform::new(0., 1.).unwrap()).unwrap();
}

pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod vec3;
