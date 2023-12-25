use std::ops::Mul;
use std::ptr::swap;
use crate::im_numbers::core::Im;

impl Mul for Im {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self {
        // if self.is_none() { return self }
        // if rhs.is_none() { return rhs }

        unsafe { self.mul_core(&mut rhs); }
        unsafe { self.collect(); }
        self
    }
}

impl Im {
    pub(crate) unsafe fn mul_core(&mut self, rhs: &mut Self) {
        self.pair_checker();
        rhs.pair_checker();

        self.mul_logic(rhs);

        if self.is_mixed_base_simple() {
            self.mixed_base_to_simple()
        }
    }

    unsafe fn mul_logic(&mut self, rhs: &mut Self) {
        if self.is_simple_logic(rhs) { self.mul_simple_logic(rhs) }
        else if self.is_mixed_base_logic(rhs) { self.mul_mixed_base_logic(rhs) }
        else if self.is_mixed_pow_logic(rhs) { self.mul_mixed_pow_logic(rhs) }
        else if self.is_mixed_mul_logic(rhs) { self.mul_mixed_mul_logic(rhs) }
        else if self.is_mixed_all() && rhs.is_mixed_all() { self.mul_mixed_super_logic(rhs) }
    }

    fn mul_simple_logic(&mut self, rhs: &Self) {

        if (self.is_real() && rhs.is_real()) || (self.is_simple_im() && rhs.is_simple_im()) {
            self.real *= rhs.real;
            self.im_pow += rhs.im_pow;
            if self.is_simple_im() { self.pair_checker() }
            if self.real == 0.0 { *self = Self::default() }
        }

        else if (self.is_real() && rhs.is_simple_im()) || (self.is_simple_im() && rhs.is_real()) {
            if self.is_zero() || rhs.is_zero() {
                *self = Self::default()
            } else {
                self.real *= rhs.real;
                if self.is_real() { self.im_pow = rhs.im_pow }
            }
        }
    }

    unsafe fn mul_mixed_base_logic(&mut self, rhs: &mut Self) {

        if self.mixed_base.is_none() && rhs.mixed_base.is_some() {
            self.simple_to_mixed_base();
        }
        else if self.mixed_base.is_some() && rhs.mixed_base.is_none() {
            rhs.simple_to_mixed_base();
        }

        Self::mul_vec(&mut self.mixed_base, &mut rhs.mixed_base);

        if self.real_mixed_base().is_some_and(|n| n == 0.0) {
            *self = Self::default()
        }
    }

    unsafe fn mul_mixed_pow_logic(&mut self, rhs: &mut Self) {
        if self.is_mixed_pow_variable(rhs) {

            if self.mixed_pow.is_none() && rhs.mixed_pow.is_some() {
                self.push_in_mixed_pow(Self::new(1.0, 0.0));
            }
            else if self.mixed_pow.is_some() && rhs.mixed_pow.is_none() {
                rhs.push_in_mixed_pow(Self::new(1.0, 0.0));
            }

            Self::add_vec(&mut self.mixed_pow, &mut rhs.mixed_pow);
        }
    }

    unsafe fn mul_mixed_mul_logic(&mut self, rhs: &mut Self) {
        if self.is_mixed_mul_variable(rhs) {

            if self.is_real() || self.is_simple_im() {
                rhs.push_in_mixed_mul(Self::new(self.real, self.im_pow));
                self.clear_simples();
                swap(self, rhs);
            }

            else if rhs.is_real() || rhs.is_simple_im() {
                self.push_in_mixed_mul(Self::new(rhs.real, rhs.im_pow));
                rhs.clear_simples();
            }

            else if self.is_mixed_im() && rhs.is_mixed_pow_and_base_only() && self.mixed_base != rhs.mixed_base {
                rhs.push_in_mixed_mul(self.clone());
                swap(self, rhs);
            }

            else if rhs.is_mixed_im() && self.is_mixed_pow_and_base_only() && self.mixed_base != rhs.mixed_base {
                self.push_in_mixed_mul(rhs.clone());
            }

            else if self == rhs {

            }
        }
    }

    fn mul_mixed_super_logic(&mut self, rhs: &mut Self) {
        let mut expr = Self::default();
        expr.push_in_mixed_base(self.clone());
        if self == rhs {
            expr.push_in_mixed_pow(Self::new(2.0, 0.0));
        }
        else {
            expr.push_in_mixed_base(rhs.clone());
        }
        *self = expr
    }

    unsafe fn mul_vec(mut lhs: &mut Option<Vec<Im>>, mut rhs: &mut Option<Vec<Im>>) {
        let is_gr = Im::is_vec_greater(lhs, rhs);
        if !is_gr { swap(lhs, rhs) }

        let mut exprs = Vec::<Im>::new();

        if let Some(v1) = &mut lhs &&
            let Some(v2) = &mut rhs
        {
            for e1 in v1.iter_mut() {
                for e2 in v2.iter_mut() {
                    Im::mul_core(e1, e2);
                    if !e1.is_zero() {
                        exprs.push(e1.clone())
                    }
                }
            }

            *lhs = Some(exprs);
        }
    }

    pub(crate) unsafe fn collect(&mut self) {
        if self.is_mixed_base_only() {
            let e = &mut Im::default();

            if let Some(v) = &mut self.mixed_base {
                while !v.is_empty() {
                    if let Some(e1) = v.pop().as_mut() {
                        e.add_core(e1);
                    }
                }
            }
            swap(self, e);
        }
    }

}
