pub fn is_equal(a: &f32, b: &f32, precision: f32) -> bool {
    if (a - b).abs() <= precision { return true }
    false
}
