pub mod default {
    pub const PRECISION: f64 = 0.0001;
}

pub trait AdvancedEQ {
    fn is_equal(&self, other: Self, precision: f64) -> bool;
}
impl AdvancedEQ for f64 {
    fn is_equal(&self, other: Self, precision: f64) -> bool {
        if (self - other).abs() <= precision.abs() { return true }
        false
    }
}
impl AdvancedEQ for f32 {
    fn is_equal(&self, other: Self, precision: f64) -> bool {
        if (self - other).abs() <= precision.abs() as f32 { return true }
        false
    }
}
