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
