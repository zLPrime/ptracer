pub fn get_random_float_neg_pos(multiplier: f32) -> f32 {
    (get_random_float() - 0.5) * 2. * multiplier
}

pub fn get_random_float() -> f32 {
    fastrand::f32()
}