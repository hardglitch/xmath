use std::ops::Mul;
use std::ptr::swap;
use crate::im_numbers::core::Im;


impl Mul for Im {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self {
        // if self.is_none() { return self }
        // if rhs.is_none() { return rhs }

        unsafe { self.mul_core(&mut rhs); }
        self
    }
}

impl Im {
    pub(crate) unsafe fn mul_core(&mut self, rhs: &mut Self) {
        self.im_pow_fixer();
        rhs.im_pow_fixer();

        self.mul_logic(rhs);

        self.pow_fixer();
        self.mul_fixer();
        self.simple_fixer();

        unsafe { self.collect(); }
    }

    unsafe fn mul_logic(&mut self, rhs: &mut Self) {
        if self.is_zero() || rhs.is_zero() { self.mul_fast_logic() }
        else if self.is_simple_logic(rhs) { self.mul_simple_logic(rhs) }
        else if self.is_mixed_base_logic(rhs) { self.mul_mixed_base_logic(rhs) }
        else if self.is_mixed_pow_logic(rhs) { self.mul_mixed_pow_logic(rhs) }
        else if self.is_mixed_mul_logic(rhs) { self.mul_mixed_mul_logic(rhs) }
    }

    fn mul_fast_logic(&mut self) {
        *self = Self::default();
    }

    fn mul_simple_logic(&mut self, rhs: &Self) {

        // Sr * Sr , Si * Si
        if self.is_sr_sr(rhs) || self.is_si_si(rhs) {
            self.real *= rhs.real;
            self.im_pow += rhs.im_pow;
            if self.is_simple_im() { self.im_pow_fixer() }
            if self.real == 0.0 { *self = Self::default() }
        }

        // Sr * Si , Si * Sr
        else if self.is_sr_si(rhs) || self.is_si_sr(rhs) {
            if self.is_zero() || rhs.is_zero() {
                *self = Self::default()
            } else {
                self.real *= rhs.real;
                if self.is_real() { self.im_pow = rhs.im_pow }
            }
        }
    }

    unsafe fn mul_mixed_base_logic(&mut self, rhs: &mut Self) {

        // a * S
        if self.is_a_s(rhs) {
            rhs.simple_to_mixed_base();
        }

        // S * a
        else if self.is_s_a(rhs) {
            self.simple_to_mixed_base();
        }

        // a * a
        Self::mul_vec(&mut self.mixed_base, &mut rhs.mixed_base);

        if self.simple_mixed_base().is_some_and(|n| n.is_zero()) {
            *self = Self::default()
        }
    }

    unsafe fn mul_mixed_pow_logic(&mut self, rhs: &mut Self) {

        // a^n * a , a^n * a^n , a^n1 * a^n2
        if self.is_an_a(rhs) || self.is_an_an(rhs) || self.is_an1_an2(rhs)
        {
            self.add_ass_mixed_pow(rhs);
        }

        // a * a^n
        else if self.is_a_an(rhs)
        {
            swap(self, rhs);
            self.add_ass_mixed_pow(rhs);
        }

        // a^n * S , a^n * b , a^n * b^n (any n)
        else if self.is_an_s(rhs) || self.is_an_b(rhs) || self.is_an_bn(rhs) {
            self.mul_ass_mixed_mul(rhs);
        }

        // S * a^n , b * a^n
        else if self.is_s_an(rhs) || self.is_b_an(rhs) {
            swap(self, rhs);
            self.mul_ass_mixed_mul(rhs);
        }
    }

    unsafe fn mul_mixed_mul_logic(&mut self, rhs: &mut Self) {

        // Ma^n * S , Ma^n * b , Ma^n * Mb^n
        if self.is_man_s(rhs) || self.is_man_b(rhs) || self.is_man_mbn(rhs) {
            self.mul_ass_mixed_mul(rhs);
        }

        // S * Ma^n , b * Ma^n
        else if self.is_man_s(rhs) || self.is_man_b(rhs) {
            swap(self, rhs);
            self.mul_ass_mixed_mul(rhs);
        }

        // Ma^n * a , Ma^n * a^n , Ma^n * b^n
        else if self.is_man_a(rhs) || self.is_man_an(rhs) || self.is_man_bn(rhs) {
            self.add_ass_mixed_pow(rhs);
        }

        // a * Ma^n , a^n * Ma^n , b^n * Ma^n
        else if self.is_a_man(rhs) || self.is_an_man(rhs) || self.is_bn_man(rhs) {
            swap(self, rhs);
            self.add_ass_mixed_pow(rhs);
        }

        // Ma^n * Ma^n , Ma^n1 * Ma^n2
        else if self.is_man_man(rhs) || self.is_man1_man2(rhs) {
            self.add_ass_mixed_pow(rhs);
            self.mul_ass_mixed_mul(rhs);
        }
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
}
