use std::cmp::Ordering;
use crate::utils::{AdvancedEQ, default};

#[derive(Debug, Clone, Copy, Default)]
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
impl Eq for ImNumber {}
impl Ord for ImNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.real.is_equal(other.real, default::PRECISION) { return Ordering::Equal }
        else if self.real > other.real { return Ordering::Greater }
        Ordering::Less
    }
}
impl PartialOrd<Self> for ImNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
