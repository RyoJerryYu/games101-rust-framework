use rand::Rng;
pub const EPSILON: f32 = 0.0001;

pub fn get_random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
