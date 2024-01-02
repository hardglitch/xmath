use crate::im::core::{Im, Sign};

impl Im {
    pub fn is_zero(&self) -> bool {
        //! Returns True if Im is zero, False otherwise.
        //! # Example
        //! ```
        //! use xmath::im::cast::ImValue;
        //!
        //! assert!(0.i().is_zero());
        //! ```

        self.is_simple() && self.real == 0.0
    }
    pub fn is_none(&self) -> bool {
        //! Returns True if Im is None, False otherwise.
        //! None is a special Im value that indicates division by zero within an expression.
        //! # Example
        //! ```
        //! use xmath::im::cast::ImValue;
        //!
        //! assert!(!0.i().is_none());
        //! ```

        self.mixed_base.as_ref().is_some_and(|v| v.is_empty()) &&
            self.mixed_pow.as_ref().is_some_and(|v| v.is_empty()) &&
            self.mixed_mul.as_ref().is_some_and(|v| v.is_empty())
    }
    pub(crate) fn is_real(&self) -> bool {
        self.mixed_base.is_none() &&
        self.mixed_pow.is_none() &&
        self.mixed_mul.is_none() &&
        self.im_pow == 0.0
    }
    pub(crate) fn is_simple_im(&self) -> bool {
        self.mixed_base.is_none() &&
        self.mixed_pow.is_none() &&
        self.mixed_mul.is_none() &&
        self.im_pow != 0.0
    }
    pub(crate) fn is_simple(&self) -> bool {
        self.is_real() || self.is_simple_im()
    }
    pub(crate) fn is_vec_greater(lhs: &Option<Vec<Im>>, rhs: &Option<Vec<Im>>) -> bool {
        if let Some(v1) = lhs &&
            let Some(v2) = rhs && v1.len() > v2.len()
        { return true }
        false
    }
    #[allow(dead_code)]
    pub(crate) unsafe fn is_equal_by_abs(&self, other: &Self) -> Sign {
        let mut neg_self = self.clone();
        neg_self.neg();
        if self == other { Sign::Plus }
        else if &neg_self == other { Sign::Minus }
        else { Sign::None }
    }
    #[allow(dead_code)]
    pub(crate) fn is_mixed_base_len_big(&self) -> bool {
        self.mixed_base.as_ref().is_some_and(|e| e.len() > 1)
    }
    #[allow(dead_code)]
    pub(crate) fn is_mixed_base_len_bigger(&self, other: &Self) -> bool {
        if let Some(b1) = &self.mixed_base &&
            let Some(b2) = &other.mixed_base &&
            b1.len() > b2.len()
        { return true }
        false
    }
    pub(crate) fn is_mixed_base_only(&self) -> bool {
        self.mixed_mul.is_none() && self.mixed_pow.is_none() && self.mixed_base.is_some() &&
            self.real == 0.0 && self.im_pow == 0.0
    }
    pub(crate) fn is_mixed_pow_and_base_only(&self) -> bool {
        self.mixed_mul.is_none() && self.mixed_pow.is_some() && self.mixed_base.is_some() &&
            self.real == 0.0 && self.im_pow == 0.0
    }
    pub(crate) fn is_mixed_all(&self) -> bool {
        self.mixed_mul.is_some() && self.mixed_pow.is_some() && self.mixed_base.is_some() &&
            self.real == 0.0 && self.im_pow == 0.0
    }
    pub(crate) fn is_fast_logic1(&self, rhs: &Self) -> bool {
        self == rhs || self.is_zero() || rhs.is_zero()
    }
    pub(crate) fn is_fast_logic2(&self, rhs: &Self) -> bool {
        self.is_zero() || rhs.is_zero()
    }
    pub(crate) fn is_simple_logic(&self, rhs: &Self) -> bool {
        self.is_simple() && rhs.is_simple()
    }
    pub(crate) fn is_mixed_base_logic(&self, rhs: &Self) -> bool {
        (self.is_mixed_base_only() && rhs.is_simple()) ||
            (rhs.is_mixed_base_only() && self.is_simple()) ||
            (self.is_mixed_base_only() && rhs.is_mixed_base_only())
    }
    pub(crate) fn is_mixed_pow_logic(&self, rhs: &Self) -> bool {
        (self.is_mixed_pow_and_base_only() && rhs.is_simple()) ||
            (rhs.is_mixed_pow_and_base_only() && self.is_simple()) ||
            (self.is_mixed_pow_and_base_only() && rhs.is_mixed_base_only()) ||
            (rhs.is_mixed_pow_and_base_only() && self.is_mixed_base_only()) ||
            (self.is_mixed_pow_and_base_only() && rhs.is_mixed_pow_and_base_only())
    }
    pub(crate) fn is_mixed_mul_logic(&self, rhs: &Self) -> bool {
        (self.is_mixed_all() && rhs.is_simple()) ||
            (rhs.is_mixed_all() && self.is_simple()) ||
            (self.is_mixed_all() && rhs.is_mixed_base_only()) ||
            (rhs.is_mixed_all() && self.is_mixed_base_only()) ||
            (self.is_mixed_all() && rhs.is_mixed_pow_and_base_only()) ||
            (rhs.is_mixed_all() && self.is_mixed_pow_and_base_only()) ||
            (self.is_mixed_all() && rhs.is_mixed_all())
    }
    pub(crate) fn is_sr_sr(&self, rhs: &Self) -> bool {
        self.is_real() && rhs.is_real()
    }
    pub(crate) fn is_si_si(&self, rhs: &Self) -> bool {
        self.is_simple_im() && rhs.is_simple_im()
    }
    pub(crate) fn is_sr_si(&self, rhs: &Self) -> bool {
        self.is_real() && rhs.is_simple_im()
    }
    pub(crate) fn is_si_sr(&self, rhs: &Self) -> bool {
        rhs.is_sr_si(self)
    }
    pub(crate) fn is_a_s(&self, rhs: &Self) -> bool {
        self.is_mixed_base_only() && rhs.is_simple()
    }
    pub(crate) fn is_s_a(&self, rhs: &Self) -> bool {
        rhs.is_a_s(self)
    }
    pub(crate) fn is_a_an(&self, rhs: &Self) -> bool {
        self.is_mixed_base_only() && rhs.is_mixed_pow_and_base_only() && self.mixed_base == rhs.mixed_base
    }
    pub(crate) fn is_an_a(&self, rhs: &Self) -> bool {
        rhs.is_a_an(self)
    }
    pub(crate) fn is_an_s(&self, rhs: &Self) -> bool {
        self.is_mixed_pow_and_base_only() && rhs.is_simple()
    }
    pub(crate) fn is_s_an(&self, rhs: &Self) -> bool {
        rhs.is_an_s(self)
    }
    pub(crate) fn is_an_x(&self, rhs: &Self) -> bool {
        self.is_mixed_pow_and_base_only() && rhs.is_mixed_base_only() && self.mixed_base != rhs.mixed_base
    }
    pub(crate) fn is_x_an(&self, rhs: &Self) -> bool {
        rhs.is_an_x(self)
    }
    pub(crate) fn is_an_an(&self, rhs: &Self) -> bool {
        rhs.is_mixed_pow_and_base_only() && self == rhs
    }
    pub(crate) fn is_an_ax(&self, rhs: &Self) -> bool {
        rhs.is_mixed_pow_and_base_only() && self.is_mixed_pow_and_base_only() &&
        self.mixed_base == rhs.mixed_base && self.mixed_pow != rhs.mixed_pow
    }
    pub(crate) fn is_an_xx(&self, rhs: &Self) -> bool {
        self.is_mixed_pow_and_base_only() && rhs.is_mixed_pow_and_base_only() &&
            self != rhs
    }
    pub(crate) fn is_man_s(&self, rhs: &Self) -> bool {
        self.is_mixed_all() && rhs.is_simple()
    }
    pub(crate) fn is_s_man(&self, rhs: &Self) -> bool {
        rhs.is_man_s(self)
    }
    pub(crate) fn is_man_a(&self, rhs: &Self) -> bool {
        self.is_mixed_all() && rhs.is_mixed_base_only() && self.mixed_base == rhs.mixed_base
    }
    pub(crate) fn is_a_man(&self, rhs: &Self) -> bool {
        rhs.is_man_a(self)
    }
    pub(crate) fn is_man_x(&self, rhs: &Self) -> bool {
        self.is_mixed_all() && rhs.is_mixed_base_only() && self.mixed_base != rhs.mixed_base
    }
    pub(crate) fn is_x_man(&self, rhs: &Self) -> bool {
        rhs.is_man_x(self)
    }
    pub(crate) fn is_man_an(&self, rhs: &Self) -> bool {
        self.is_mixed_all() && rhs.is_mixed_pow_and_base_only() &&
        self.mixed_base == rhs.mixed_base && self.mixed_pow == rhs.mixed_pow
    }
    pub(crate) fn is_an_man(&self, rhs: &Self) -> bool {
        rhs.is_man_an(self)
    }
    pub(crate) fn is_man_ax(&self, rhs: &Self) -> bool {
        self.is_mixed_all() && rhs.is_mixed_pow_and_base_only() &&
        self.mixed_base == rhs.mixed_base && self.mixed_pow != rhs.mixed_pow
    }
    pub(crate) fn is_ax_man(&self, rhs: &Self) -> bool {
        rhs.is_man_ax(self)
    }
    pub(crate) fn is_man_xx(&self, rhs: &Self) -> bool {
        self.is_mixed_all() && rhs.is_mixed_pow_and_base_only() &&
        self.mixed_base != rhs.mixed_base && self.mixed_pow != rhs.mixed_pow
    }
    pub(crate) fn is_xx_man(&self, rhs: &Self) -> bool {
        rhs.is_man_xx(self)
    }
    pub(crate) fn is_man_man(&self, rhs: &Self) -> bool {
        self.is_mixed_all() && self == rhs
    }
    pub(crate) fn is_man_xax(&self, rhs: &Self) -> bool {
        self.is_mixed_all() && rhs.is_mixed_all() && self.mixed_base == rhs.mixed_base &&
        self.mixed_mul != rhs.mixed_mul && self.mixed_pow != rhs.mixed_pow
    }
    pub(crate) fn is_man_xxx(&self, rhs: &Self) -> bool {
        self.is_mixed_all() && rhs.is_mixed_all() && self != rhs
    }
}