use std::ops::Div;
use std::ptr::swap;
use crate::im::core::Im;

impl Div for Im {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self {
        if self.is_none() || rhs.is_none() { return Self::none() }

        unsafe { self.div_core(&mut rhs); }
        self
    }
}

impl Im {
    pub(crate) unsafe fn div_core(&mut self, rhs: &mut Self) {
        self.im_pow_fixer();
        rhs.im_pow_fixer();

        self.div_logic(rhs);
        if self.is_none() || rhs.is_none() {
            *self = Self::none();
            return
        }

        self.fixer_pack();
        unsafe { self.collect(); }
    }

    unsafe fn div_logic(&mut self, rhs: &mut Self) {
        if self.is_fast_logic1(rhs) { self.div_fast_logic(rhs) }
        else if self.is_simple_logic(rhs) { self.div_simple_logic(rhs) }
        else if self.is_mixed_base_logic(rhs) { self.div_mixed_base_logic(rhs) }
        else if self.is_mixed_pow_logic(rhs) { self.div_mixed_pow_logic(rhs) }
        else if self.is_mixed_mul_logic(rhs) { self.div_mixed_mul_logic(rhs) }
    }

    fn div_fast_logic(&mut self, rhs: &Self) {
        if rhs.is_zero() {
            *self = Self::none();
        }
        else if self.is_zero() {
            *self = Self::default()
        }
        else if self == rhs {
            *self = Self::new(1.0, 0.0);
        }
    }

    fn div_simple_logic(&mut self, rhs: &Self) {

        // Sr / Sr , Si / Si
        if self.is_sr_sr(rhs) || self.is_si_si(rhs) {
            self.real /= rhs.real;
            self.im_pow -= rhs.im_pow;
            if self.is_simple_im() { self.im_pow_fixer() }
        }

        // Sr / Si , Si / Sr
        else if self.is_sr_si(rhs) || self.is_si_sr(rhs) {
            self.real /= rhs.real;
            if self.is_real() { self.im_pow = -rhs.im_pow }
        }
    }

    unsafe fn div_mixed_base_logic(&mut self, rhs: &mut Self) {

        // a / S
        if self.is_a_s(rhs) {
            rhs.simple_to_mixed_base();
            Self::div_vec(&mut self.mixed_base, &mut rhs.mixed_base);
        }

        // S / a , a / x , x / a
        else {
            rhs.pow_neg();
            swap(self, rhs);
            self.mul_ass_mixed_mul(rhs);
        }

        if self.simple_mixed_base().is_some_and(|n| n.is_zero()) {
            *self = Self::default()
        }
    }

    unsafe fn div_mixed_pow_logic(&mut self, rhs: &mut Self) {

        // a^n / a , a^n / a^x
        if self.is_an_a(rhs) || self.is_an_ax(rhs)
        {
            self.sub_ass_mixed_pow(rhs);
        }

        // a / a^n
        else if self.is_a_an(rhs)
        {
            swap(self, rhs);
            self.sub_ass_mixed_pow(rhs);
        }

        // a^n / S , a^n / x
        else if self.is_an_s(rhs) || self.is_an_x(rhs) {
            rhs.simple_to_mixed_base();
            rhs.push_in_mixed_pow(Self::new(-1.0, 0.0));
            self.push_in_mixed_mul(rhs.clone());
        }

        // S / a^n , x / a^n
        else if self.is_s_an(rhs) || self.is_x_an(rhs) {
            swap(self, rhs);
            rhs.simple_to_mixed_base();
            rhs.push_in_mixed_pow(Self::new(-1.0, 0.0));
            self.push_in_mixed_mul(rhs.clone());
        }

        // a^n / x^x
        else if self.is_an_xx(rhs) {
            rhs.push_in_mixed_pow(Self::new(-1.0, 0.0));
            rhs.push_in_mixed_mul(self.clone());
            swap(self, rhs);
        }
    }

    unsafe fn div_mixed_mul_logic(&mut self, rhs: &mut Self) {

        // Ma^n / S
        if self.is_man_s(rhs) {
            rhs.simple_to_mixed_base();
            rhs.push_in_mixed_pow(Self::new(-1.0, 0.0));
            self.mul_ass_mixed_mul(rhs);
        }

        // S / Ma^n
        else if self.is_s_man(rhs) {
            swap(self, rhs);
            rhs.simple_to_mixed_base();
            rhs.push_in_mixed_pow(Self::new(-1.0, 0.0));
            self.mul_ass_mixed_mul(rhs);
        }

        // Ma^n / a , Ma^n / a^n , Ma^n / a^x
        else if self.is_man_a(rhs) || self.is_man_an(rhs) || self.is_man_ax(rhs) {
            self.sub_ass_mixed_pow(rhs);
        }

        // a / Ma^n , a^n / Ma^n , a^x / Ma^n
        else if self.is_a_man(rhs) || self.is_an_man(rhs) || self.is_ax_man(rhs) {
            swap(self, rhs);
            self.pow_neg();
            if self.is_none() { return }
            self.add_ass_mixed_pow(rhs);
        }

        // Ma^n / Xa^x
        else if self.is_man_xax(rhs) {
            self.sub_ass_mixed_pow(rhs);
            self.div_ass_mixed_mul(rhs);
        }

        // Ma^n / x , Ma^n / x^x , Ma^n / Xx^x
        else if self.is_man_x(rhs) || self.is_man_xx(rhs) || self.is_man_xxx(rhs) {
            self.sub_ass_mixed_pow(rhs);
        }

        //  x / Ma^n , x^x / Ma^n
        else if self.is_x_man(rhs) || self.is_xx_man(rhs) {
            swap(self, rhs);
            self.pow_neg();
            if self.is_none() { return }
            self.mul_ass_mixed_mul(rhs);
        }
    }

    unsafe fn div_vec(mut lhs: &mut Option<Vec<Im>>, mut rhs: &mut Option<Vec<Im>>) {
        let is_gr = Im::is_vec_greater(lhs, rhs);
        if !is_gr { swap(lhs, rhs) }

        let mut exprs = Vec::<Im>::new();

        if let Some(v1) = &mut lhs &&
            let Some(v2) = &mut rhs
        {
            for e1 in v1.iter_mut() {
                for e2 in v2.iter_mut() {
                    Im::div_core(e1, e2);
                    if !e1.is_zero() {
                        exprs.push(e1.clone())
                    }
                }
            }

            *lhs = Some(exprs);
        }
    }
}
