use crate::utils::{AdvancedEQ, default};

#[derive(Debug, Clone, Default)]
pub(crate) struct ImNumber {
    pub(crate) real: f64,
    pub(crate) im_pow: f64,
}
impl PartialEq for ImNumber {
    fn eq(&self, other: &Self) -> bool {
        self.real.is_equal(other.real, default::PRECISION) &&
            self.im_pow.is_equal(other.im_pow, default::PRECISION)
    }
}
impl ImNumber {
    pub(crate) fn new(real: f64, im_pow: f64) -> Self {
        Self { real, im_pow }
    }

    pub(crate) fn pair_checker(&mut self) {
        let pairs = (self.im_pow / 2.0).trunc();
        if pairs != 0.0 && pairs % 2.0 != 0.0 { self.real = -self.real }
        self.im_pow -= 2.0 * pairs;
    }

    pub(crate) fn is_real(&self) -> bool {
        self.im_pow == 0.0
    }

    pub(crate) fn is_equal_by_abs(&self, other: &Self) -> bool {
        self.real.abs() == other.real.abs() && self.im_pow == other.im_pow
    }
}
