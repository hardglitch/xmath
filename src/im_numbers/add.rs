use std::ops::Add;
use std::ptr::swap;
use crate::im_numbers::core::Im;

impl Add for Im {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self {
        // if self.is_none() { return self }
        // if rhs.is_none() { return rhs }

        unsafe { self.add_core(&mut rhs); }
        self
    }
}

impl Im {
    pub(crate) unsafe fn add_core(&mut self, rhs: &mut Self) {
        self.pair_checker();
        rhs.pair_checker();

        self.add_logic(rhs);

        if self.is_mixed_base_simple() {
            self.mixed_base_to_simple()
        }
    }

    unsafe fn add_logic(&mut self, rhs: &mut Self) {
        if self.is_simple_logic(rhs) { self.add_simple_logic(rhs) }
        else if self.is_mixed_base_logic(rhs) { self.add_mixed_base_logic(rhs) }
        else if self.is_mixed_mul_logic(rhs) { self.add_mixed_mul_logic(rhs) }
        else if self.is_mixed_all() && rhs.is_mixed_all() { self.add_mixed_super_logic(rhs) }
    }

    fn add_simple_logic(&mut self, rhs: &Self) {

        if (self.is_real() && rhs.is_real()) || (self.is_simple_im() && rhs.is_simple_im()) {
            self.real += rhs.real;
            if self.real == 0.0 { self.im_pow = 0.0 }
        }

        else if (self.is_real() && rhs.is_simple_im()) || (self.is_simple_im() && rhs.is_real()) {
            if self.is_zero() && rhs.is_zero() {
                *self = Self::default();
            }
            else {
                if !self.is_zero() {
                    self.simple_to_mixed_base();
                }
                if !rhs.is_zero() {
                    self.push_in_mixed_base(rhs.clone());
                }
            }
        }
    }

    unsafe fn add_mixed_base_logic(&mut self, rhs: &mut Self) {

        if (self.is_real() || self.is_simple_im()) && rhs.mixed_base.is_some() {
            self.simple_to_mixed_base()
        }

        else if self.mixed_base.is_some() && (rhs.is_real() || rhs.is_simple_im()) {
            rhs.simple_to_mixed_base()
        }

        Self::add_vec(&mut self.mixed_base, &mut rhs.mixed_base);

        if self.real_mixed_base().is_some_and(|n| n == 0.0) {
            *self = Self::default()
        };
    }

    unsafe fn add_mixed_mul_logic(&mut self, rhs: &mut Self) {
        if self.is_mixed_mul_variable(rhs) {

            if self.mixed_mul.is_none() && rhs.mixed_mul.is_none() {
                let e = Self::new(2.0, 0.0);
                self.push_in_mixed_mul(e);
            }

            else if self.mixed_mul.is_none() && rhs.mixed_mul.is_some() {
                swap(&mut self.mixed_mul, &mut rhs.mixed_mul);
            }

            else if let Some(mut m1) = self.real_mixed_mul() &&
                let Some(m2) = rhs.real_mixed_mul()
            {
                m1 += m2;
                if m1 == 0.0 { *self = Self::default() };
            }

            else {
                Self::add_vec(&mut self.mixed_mul, &mut rhs.mixed_mul);
            }
        }
    }

    fn add_mixed_super_logic(&mut self, rhs: &mut Self) {
        let mut expr = Self::default();
        expr.push_in_mixed_base(self.clone());
        if self == rhs {
            expr.push_in_mixed_mul(Self::new(2.0, 0.0));
        }
        else {
            expr.push_in_mixed_base(rhs.clone());
        }
        *self = expr
    }

    pub(crate) unsafe fn add_vec(mut lhs: &mut Option<Vec<Im>>, mut rhs: &mut Option<Vec<Im>>) {

        let is_gr = Im::is_vec_greater(lhs, rhs);
        if !is_gr { swap(lhs, rhs) }

        let mut exprs = Vec::<Im>::new();

        if let Some(v1) = &mut lhs &&
            let Some(v2) = &mut rhs
        {
            for e1 in v1.iter_mut() {
                for e2 in v2.iter_mut() {
                    if e1.im_pow == e2.im_pow {
                        Im::add_core(e1, e2);
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
