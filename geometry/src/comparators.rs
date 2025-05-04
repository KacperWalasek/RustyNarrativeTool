const EPS: f32 = 0.0001;

pub fn are_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
}

pub fn less_or_equal(a: f32, b: f32) -> bool {
    are_equal(a, b) || a < b
}

pub fn is_zero(a: f32) -> bool {
    a < EPS
}
