pub fn is_equal(a: &f64, b: &f64, precision: f64) -> bool
{
    if (a - b).abs() <= precision.abs() { return true }
    false
}
