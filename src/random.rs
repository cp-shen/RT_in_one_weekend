use crate::vec3::*;
use rand::Rng;

pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_float_range(a: f32, b: f32) -> f32 {
    assert!(a < b, "{},{} is not valid for random range", a, b);
    let mut rng = rand::thread_rng();
    rng.gen_range(a, b)
}

pub fn random_in_unit_shpere() -> Vec3 {
    loop {
        let x = random_float_range(-1.0, 1.0);
        let y = random_float_range(-1.0, 1.0);
        let z = random_float_range(-1.0, 1.0);
        let v = Vec3::new(x, y, z);
        if v.length_sqared() < 1.0 {
            return v;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a = random_float_range(0.0, 2.0 * std::f32::consts::PI);
    let z = random_float_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();

    let x = r * a.cos();
    let y = r * a.sin();
    Vec3::new(x, y, z)
}
