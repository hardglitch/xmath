use std::ops::Sub;
use std::ptr::swap;
use crate::im_numbers::core::Im;

impl Sub for Im {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self {
        // if self.is_none() { return self }
        // if rhs.is_none() { return rhs }

        unsafe { self.sub_core(&mut rhs); }
        self
    }
}

impl Im {
    pub(crate) unsafe fn sub_core(&mut self, rhs: &mut Self) {
        self.pair_checker();
        rhs.pair_checker();

        self.sub_logic(rhs);

        if self.is_mixed_base_simple() {
            self.mixed_base_to_simple()
        }
    }

    unsafe fn sub_logic(&mut self, rhs: &mut Self) {
        if self.is_simple_logic(rhs) { self.sub_simple_logic(rhs) }
        else if self.is_mixed_base_logic(rhs) { self.sub_mixed_base_logic(rhs) }
        else if self.is_mixed_mul_logic(rhs) { self.sub_mixed_mul_logic(rhs) }
    }

    fn sub_simple_logic(&mut self, rhs: &mut Self) {

        if (self.is_real() && rhs.is_real()) || (self.is_simple_im() && rhs.is_simple_im()) {
            self.real -= rhs.real;
            if self.real == 0.0 { *self = Self::default() }
        }

        else if (self.is_real() && rhs.is_simple_im()) || (self.is_simple_im() && rhs.is_real()) {
            if self.is_zero() && rhs.is_zero() {
                *self = Self::default()
            } else {
                if !self.is_zero() {
                    self.simple_to_mixed_base();
                }
                if !rhs.is_zero() {
                    let mut expr = rhs.clone();
                    expr.neg();
                    self.push_in_mixed_base(expr);
                }
            }
        }
    }

    unsafe fn sub_mixed_base_logic(&mut self, rhs: &mut Self) {

        if (self.is_real() || self.is_simple_im()) && rhs.mixed_base.is_some() {
            self.simple_to_mixed_base()
        }
        else if self.mixed_base.is_some() && (rhs.is_real() || rhs.is_simple_im()) {
            rhs.simple_to_mixed_base()
        }

        Self::sub_vec(&mut self.mixed_base, &mut rhs.mixed_base);

        if self.real_mixed_base().is_some_and(|n| n == 0.0) {
            *self = Self::default()
        };
    }

    unsafe fn sub_mixed_mul_logic(&mut self, rhs: &mut Self) {
        if self.is_mixed_mul_variable(rhs) {

            if self.mixed_mul.is_none() && rhs.mixed_mul.is_none() {
                *self = Self::default();
            }

            else if self.mixed_mul.is_none() && rhs.mixed_mul.is_some() {
                swap(&mut self.mixed_mul, &mut rhs.mixed_mul);
                self.neg();
            }

            else if let Some(mut m1) = self.real_mixed_mul() &&
                let Some(m2) = rhs.real_mixed_mul()
            {
                m1 -= m2;
                if m1 == 0.0 { *self = Self::default() };
            }

            else {
                Self::sub_vec(&mut self.mixed_mul, &mut rhs.mixed_mul);
            }
        }
    }

    pub(crate) unsafe fn sub_vec(mut lhs: &mut Option<Vec<Im>>, mut rhs: &mut Option<Vec<Im>>) {
        let is_gr = Im::is_vec_greater(lhs, rhs);
        if !is_gr { swap(lhs, rhs) }

        let mut exprs = Vec::<Im>::new();

        if let Some(v1) = &mut lhs &&
            let Some(v2) = &mut rhs
        {
            for e1 in v1.iter_mut() {
                for e2 in v2.iter_mut() {
                    if e1.im_pow == e2.im_pow {
                        Im::sub_core(e1, e2);
                    }
                    if !e1.is_zero() {
                        exprs.push(e1.clone())
                    }
                }
            }

            *lhs = Some(exprs);
        }
    }
}
