use crate::im_numbers::core::ImExpression;
use crate::im_numbers::im_number::ImNumber;

impl ImExpression {

    fn pow_zero_check(&mut self, other: &mut Self) -> bool {
        if other.is_mixed_base_zero() || other.is_mixed_mul_zero()
        {
            *self = Self::new(1.0, false);
            return true
        }
        else if self.is_mixed_base_zero() || self.is_mixed_mul_zero() {
            return true
        }
        false
    }

    pub(crate) unsafe fn div(&mut self, rhs: &mut Self) -> Option<()> {
        if rhs.is_mixed_base_zero() || rhs.is_mixed_mul_zero() { return None }

        if self.mixed_base.len() < rhs.mixed_base.len()
        {
            swap(self, rhs);
            self.mixed_pow.iter_mut().for_each(|e| e.neg());
            self.mixed_mul.iter_mut().for_each(|e| e.neg());
            rhs.mixed_pow.iter_mut().for_each(|e| e.neg());
            rhs.mixed_mul.iter_mut().for_each(|e| e.neg());
        }

        self.pair_checker();
        rhs.pair_checker();

        #[allow(irrefutable_let_patterns)]
        if let s = self.is_equal_by_abs(rhs) && s != Sign::None {
            if s == Sign::Plus {
                *self = Self::new(1.0, 0.0);
            } else {
                *self = Self::new(-1.0, 0.0);
            }
            return Some(())
        }

        else if self.mixed_base == rhs.mixed_base &&
            let Some(p1) = self.real_mixed_pow() &&
            let Some(p2) = rhs.real_mixed_pow() &&
            let Some(b) = self.real_mixed_base() && b != 0.0
        {
            if let Some(b_mut) = self.real_mixed_base_mut() {
                b_mut.real = b.mixed_powf(p1 - p2)
            }
            return Some(())
        }

        else if (self.mixed_base == rhs.mixed_base && self.mixed_pow == rhs.mixed_pow && self.is_mixed_pow_len_big()) ||
            (self.real_mixed_pow().is_none() && self.mixed_pow.is_some() && !rhs.is_im_value())
        {
            if let Some(e) = &mut self.mixed_mul {
                e.div(rhs)?;
                return Some(())
            }
        }

        else if self.mixed_base == rhs.mixed_base {
            if let Some(e) = &mut self.mixed_pow {
                e.sub(rhs);
                return Some(())
            }
            if self.is_mixed_pow_zero() {
                *self = Self::new(1.0, 0.0);
                return Some(())
            }
        }

        let div = |e1: &ImNumber, e2: &ImNumber| -> Option<ImNumber> {
            if e2.real == 0.0 { return None }
            else if e1.is_equal_by_abs(e2) {
                return Some(ImNumber::new(e1.real / e2.real, 0.0))
            } else if e1.is_real() {
                return Some(ImNumber::new(e1.real / e2.real, -e2.im_mixed_pow))
            } else if e2.is_real() {
                return Some(ImNumber::new(e1.real / e2.real, e1.im_mixed_pow))
            } else if !e1.is_real() && !e2.is_real() {
                return Some(ImNumber::new(e1.real / e2.real, 0.0))
            }
            None
        };

        let mut elems_mut = Vec::<ImNumber>::new();

        for e1 in self.mixed_base.iter() {
            for e2 in rhs.mixed_base.iter() {
                if let Some(e) = div(e1, e2) {
                    if e.real != 0.0 { elems_mut.push(e)}
                }
                else if self.mixed_base.len() == 1 && rhs.mixed_base.len() == 1 {
                    if e2.real == 0.0 { return None }
                    let mut e = Box::<ImNumber>::default();
                    if e1.is_real() { *e = *e2 } else { *e = *e1 }
                    e.real = e1.real / e2.real;
                    if e.real != 0.0 { elems_mut.push(*e)}
                }
                else if self.mixed_base.len() > 1 && rhs.mixed_base.len() == 1 && e1.real != 0.0 {
                    elems_mut.push(*e1)
                }
            }
        }

        if elems_mut.is_empty() { elems_mut.push(ImNumber { real: 0.0, im_mixed_pow: 0.0 }) }
        self.mixed_base = elems_mut;
        self.collect();
        Some(())
    }

}