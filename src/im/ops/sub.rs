use std::ops::Sub;
use std::ptr::swap;
use crate::im::core::Im;

impl Sub for Im {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self {
        if self.is_none() || rhs.is_none() { return Self::none() }

        unsafe { self.sub_core(&mut rhs); }
        self
    }
}

impl Im {
    pub(crate) unsafe fn sub_core(&mut self, rhs: &mut Self) {
        if self.is_none() || rhs.is_none() { return }

        self.im_pow_fixer();
        rhs.im_pow_fixer();

        self.sub_logic(rhs);
        if self.is_none() || rhs.is_none() {
            *self = Self::none();
            return
        }

        self.fixer_pack();
    }

    unsafe fn sub_logic(&mut self, rhs: &mut Self) {
        if self.is_fast_logic1(rhs) { self.sub_fast_logic(rhs) }
        else if self.is_simple_logic(rhs) { self.sub_simple_logic(rhs) }
        else if self.is_mixed_base_logic(rhs) { self.sub_mixed_base_logic(rhs) }
        else if self.is_mixed_pow_logic(rhs) { self.sub_mixed_pow_logic(rhs) }
        else if self.is_mixed_mul_logic(rhs) { self.sub_mixed_mul_logic(rhs) }
    }

    unsafe fn sub_fast_logic(&mut self, rhs: &mut Self) {
        if self.is_zero() {
            swap(self, rhs);
            self.neg();
        }
        else if self == rhs {
            *self = Self::default();
        }
    }

    unsafe fn sub_simple_logic(&mut self, rhs: &mut Self) {

        // Sr - Sr , Si - Si
        if self.is_sr_sr(rhs) || self.is_si_si(rhs) {
            self.real -= rhs.real;
            if self.real == 0.0 { *self = Self::default() }
        }

        // Sr - Si , Si - Sr
        else if self.is_sr_si(rhs) || self.is_si_sr(rhs) {
            self.simple_to_mixed_base();
            rhs.neg();
            self.push_in_mixed_base(rhs.clone());
        }
    }

    unsafe fn sub_mixed_base_logic(&mut self, rhs: &mut Self) {

        // a - S
        if self.is_a_s(rhs) {
            rhs.simple_to_mixed_base();
        }

        // S - a
        else if self.is_s_a(rhs) {
            self.simple_to_mixed_base();
        }

        // a - x
        Self::sub_vec(&mut self.mixed_base, &mut rhs.mixed_base);

        if self.simple_mixed_base().is_some_and(|n| n.is_zero()) {
            *self = Self::default()
        };
    }

    unsafe fn sub_mixed_pow_logic(&mut self, rhs: &mut Self) {

        // a^n - a , a^n - S , a^n - x , a^n - a^x , a^n - x^x
        if self.is_an_a(rhs) || self.is_an_s(rhs) || self.is_an_x(rhs) || self.is_an_ax(rhs) || self.is_an_xx(rhs)
        {
            rhs.neg();
            self.add_ass_mixed_base(rhs);
        }

        // a - a^n , S - a^n , x - a^n
        else if self.is_a_an(rhs) || self.is_s_an(rhs) || self.is_x_an(rhs)
        {
            swap(self, rhs);
            self.neg();
            self.add_ass_mixed_base(rhs);
        }
    }

    unsafe fn sub_mixed_mul_logic(&mut self, rhs: &mut Self) {

        // Ma^n - a^n
        if self.is_man_an(rhs) {
            self.sub_ass_mixed_mul(&mut Self::new(1.0, 0.0));
        }

        // a^n - Ma^n
        else if self.is_an_man(rhs) {
            swap(self, rhs);
            self.neg();
            self.add_ass_mixed_mul(&mut Self::new(1.0, 0.0));
        }


        // Ma^n - S , Ma^n - a , Ma^n - x , Ma^n - x^x , Ma^n - Xx^x
        else if self.is_man_s(rhs) || self.is_man_a(rhs) || self.is_man_x(rhs) || self.is_man_xx(rhs) || self.is_man_xxx(rhs)
        {
            rhs.neg();
            self.add_ass_mixed_base(rhs);
        }

        // S - Ma^n , a - Ma^n , x - Ma^n , x^x - Ma^n
        else if self.is_s_man(rhs) || self.is_a_man(rhs) || self.is_x_man(rhs) || self.is_xx_man(rhs)
        {
            swap(self, rhs);
            self.neg();
            self.add_ass_mixed_base(rhs);
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
