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
        self.im_pow_fixer();
        rhs.im_pow_fixer();

        self.add_logic(rhs);

        self.pow_fixer();
        self.mul_fixer();
        self.simple_fixer();
    }

    unsafe fn add_logic(&mut self, rhs: &mut Self) {
        if self.is_simple_logic(rhs) { self.add_simple_logic(rhs) }
        else if self.is_mixed_base_logic(rhs) { self.add_mixed_base_logic(rhs) }
        else if self.is_mixed_pow_logic(rhs) { self.add_mixed_pow_logic(rhs) }
        else if self.is_mixed_mul_logic(rhs) { self.add_mixed_mul_logic(rhs) }
    }

    fn add_simple_logic(&mut self, rhs: &Self) {

        // Sr + Sr , Si + Si
        if self.is_sr_sr(rhs) || self.is_si_si(rhs) {
            self.real += rhs.real;
            if self.real == 0.0 { self.im_pow = 0.0 }
        }

        // Sr + Si , Si + Sr
        else if self.is_sr_si(rhs) || self.is_si_sr(rhs) {
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

        // a + S
        if self.is_a_s(rhs) {
            rhs.simple_to_mixed_base();
        }

        // S + a
        else if self.is_s_a(rhs) {
            self.simple_to_mixed_base();
        }

        // a + a
        Self::add_vec(&mut self.mixed_base, &mut rhs.mixed_base);

        if self.real_mixed_base().is_some_and(|n| n == 0.0) {
            *self = Self::default()
        };
    }

    unsafe fn add_mixed_pow_logic(&mut self, rhs: &mut Self) {

        // a^n + a , a^n + S , a^n + b , a^n + b^n , a^n1 + a^n2
        if self.is_an_a(rhs) || self.is_an_s(rhs) || self.is_an_b(rhs) || self.is_an_bn(rhs) || self.is_an1_an2(rhs)
        {
            self.add_ass_mixed_base(rhs);
        }

        // a + a^n , S + a^n , b + a^n
        else if self.is_a_an(rhs) || self.is_s_an(rhs) || self.is_b_an(rhs)
        {
            swap(self, rhs);
            self.add_ass_mixed_base(rhs);
        }

        // a^n + a^n
        else if self.is_an_an(rhs) {
            self.set_mixed_mul(2.0, 0.0);
        }
    }

    unsafe fn add_mixed_mul_logic(&mut self, rhs: &mut Self) {

        // Ma^n + a^n
        if self.is_man_an(rhs) {
            self.add_ass_mixed_mul(&mut Self::new(1.0, 0.0));
        }

        // a^n + Ma^n
        else if self.is_an_man(rhs) {
            swap(self, rhs);
            self.add_ass_mixed_mul(&mut Self::new(1.0, 0.0));
        }

        // Ma^n1 + Ma^n2
        else if self.is_man_man(rhs) {
            self.add_ass_mixed_mul(rhs);
        }

        // Ma^n + S , Ma^n + a , Ma^n + b , Ma^n1 + Mb^n2 , Ma^n1 + b^n2 , Ma^n1 + a^n2 , Mb^n + Ma^n
        else if self.is_man_s(rhs) || self.is_man_a(rhs) || self.is_man_b(rhs) ||
            self.is_man_mbn(rhs) || self.is_man_bn(rhs) || self.is_man1_an2(rhs) || self.is_mbn_man(rhs)
        {
            self.add_ass_mixed_base(rhs);
        }

        // S + Ma^n , b + Ma^n , a + Ma^n , b^n1 + Ma^n2
        else if self.is_s_man(rhs) || self.is_b_man(rhs) || self.is_a_man(rhs) || self.is_bn_man(rhs)
        {
            swap(self, rhs);
            self.add_ass_mixed_base(rhs);
        }
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
