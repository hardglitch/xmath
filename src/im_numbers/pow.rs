use crate::im_numbers::core::ImExpression;

impl ImExpression {
        pub(crate) unsafe fn pow(&mut self, other: &mut Self) {
        self.pair_checker();
        other.pair_checker();

        if self.mixed_pow_zero_check(other) ||
           self.real_mixed_base().is_some_and(|n| n == 1.0)
        { return }

        if  self.mixed_mul.is_none() &&
            other.mixed_mul.is_none() &&
            self.mixed_pow.is_none() &&
            other.mixed_pow.is_none() &&
            let Some(p) = other.real_mixed_base()
        {
            if let Some(b_mut) = self.real_mixed_base_mut() {
                b_mut.real = b_mut.real.mixed_powf(p)
            }
            return
        }

        else if self.mixed_pow.is_none() && self.mixed_mul.is_none() && other.is_im_value() {
            self.mixed_pow = Some(Box::new(other.clone()))
        }

        else if self.is_im_value() {
            if !other.is_im_value() && let Some(n) = other.real_mixed_base() && n.fract().abs() == 0.0 {
                let mut stable = self.clone();
                for _ in 0..(n as i64 - 1) {
                    self.mixed_mul(&mut stable)
                }
            }
            else if let Some(p) = &mut self.mixed_pow &&
                    let Some(m) = &mut self.mixed_mul
            {
                p.mixed_mul(other);
                m.mixed_mul(other)
            }
        }

    }

}