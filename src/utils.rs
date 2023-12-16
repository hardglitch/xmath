pub trait AdvancedEQ<Other = Self> {
    type Output = bool;
    fn is_equal(&self, other: Self, precision: f64) -> Self::Output;
}

impl AdvancedEQ for f64 {
    type Output = bool;
    fn is_equal(&self, other: Self, precision: f64) -> Self::Output {
        if (self - other).abs() <= precision.abs() { return true }
        false
    }
}

impl AdvancedEQ for f32 {
    type Output = bool;
    fn is_equal(&self, other: Self, precision: f64) -> Self::Output {
        if (self - other).abs() <= precision.abs() as f32 { return true }
        false
    }
}
