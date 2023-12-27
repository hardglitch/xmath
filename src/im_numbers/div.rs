use std::ops::Div;
use std::ptr::swap;
use crate::im_numbers::core::Im;

impl Div for Im {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self {
        // if self.is_none() { return self }
        // if rhs.is_none() { return rhs }

        unsafe { self.div_core(&mut rhs); }
        self
    }
}

impl Im {
    pub(crate) unsafe fn div_core(&mut self, rhs: &mut Self) -> Option<()> {
        self.im_pow_fixer();
        rhs.im_pow_fixer();

        self.div_logic(rhs)?;

        self.pow_fixer();
        self.mul_fixer();
        self.simple_fixer();

        unsafe { self.collect(); }

        Some(())
    }

    unsafe fn div_logic(&mut self, rhs: &mut Self) -> Option<()> {
        if self == rhs { self.div_fast_logic() }
        else if self.is_simple_logic(rhs) { self.div_simple_logic(rhs) }
        else if self.is_mixed_base_logic(rhs) { self.div_mixed_base_logic(rhs) }
        // else if self.is_mixed_pow_logic(rhs) { self.div_mixed_pow_logic(rhs) }
        else if self.is_mixed_mul_logic(rhs) { self.div_mixed_mul_logic(rhs) }
        else { Some(()) }
    }

    fn div_fast_logic(&mut self) -> Option<()> {
        *self = Self::new(1.0, 0.0);
        Some(())
    }

    fn div_simple_logic(&mut self, rhs: &Self) -> Option<()> {

        if (self.is_real() && rhs.is_real()) || (self.is_simple_im() && rhs.is_simple_im()) {
            if rhs.real == 0.0 { return None }
            self.real /= rhs.real;
            self.im_pow -= rhs.im_pow;
            if self.is_simple_im() { self.im_pow_fixer() }
            if self.real == 0.0 { *self = Self::default() }
        }

        else if (self.is_real() && rhs.is_simple_im()) || (self.is_simple_im() && rhs.is_real()) {
            if self.is_zero() {
                *self = Self::default()
            } else {
                if rhs.real == 0.0 { return None }
                self.real /= rhs.real;
                if self.is_real() { self.im_pow = rhs.im_pow }
            }
        }
        Some(())
    }

    unsafe fn div_mixed_base_logic(&mut self, rhs: &mut Self) -> Option<()> {

        if rhs.is_real() && rhs.real == 1.0 { return Some(()) }

        if self.is_real() {
            // rhs.pow(-1.0); todo()
        }


        Self::div_vec(&mut self.mixed_base, &mut rhs.mixed_base);

        if self.real_mixed_base().is_some_and(|n| n == 0.0) {
            *self = Self::default()
        }
        Some(())
    }

    // unsafe fn div_mixed_pow_logic(&mut self, rhs: &mut Self) -> Option<()> {
    //
    //         Self::add_vec(&mut self.mixed_pow, &mut rhs.mixed_pow);
    //
    //     Some(())
    // }

    unsafe fn div_mixed_mul_logic(&mut self, rhs: &mut Self) -> Option<()> {
        if self.is_simple() {
            rhs.push_in_mixed_mul(Self::new(self.real, self.im_pow));
            self.clear_simples();
            swap(self, rhs);
        }

        else if rhs.is_simple() {
            self.push_in_mixed_mul(Self::new(rhs.real, rhs.im_pow));
            rhs.clear_simples();
        }

        else if rhs.is_mixed_pow_and_base_only() && self.mixed_base != rhs.mixed_base {
            rhs.push_in_mixed_mul(self.clone());
            swap(self, rhs);
        }

        else if self.is_mixed_pow_and_base_only() && self.mixed_base != rhs.mixed_base {
            self.push_in_mixed_mul(rhs.clone());
        }

        else if self == rhs {

        }
        Some(())
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

// impl ImExpression {
//
//     fn pow_zero_check(&mut self, other: &mut Self) -> bool {
//         if other.is_mixed_base_zero() || other.is_mixed_mul_zero()
//         {
//             *self = Self::new(1.0, false);
//             return true
//         }
//         else if self.is_mixed_base_zero() || self.is_mixed_mul_zero() {
//             return true
//         }
//         false
//     }
//
//     pub(crate) unsafe fn div(&mut self, rhs: &mut Self) -> Option<()> {
//         if rhs.is_mixed_base_zero() || rhs.is_mixed_mul_zero() { return None }
//
//         if self.mixed_base.len() < rhs.mixed_base.len()
//         {
//             swap(self, rhs);
//             self.mixed_pow.iter_mut().for_each(|e| e.neg());
//             self.mixed_mul.iter_mut().for_each(|e| e.neg());
//             rhs.mixed_pow.iter_mut().for_each(|e| e.neg());
//             rhs.mixed_mul.iter_mut().for_each(|e| e.neg());
//         }
//
//         self.im_pow_fixer();
//         rhs.im_pow_fixer();
//
//         #[allow(irrefutable_let_patterns)]
//         if let s = self.is_equal_by_abs(rhs) && s != Sign::None {
//             if s == Sign::Plus {
//                 *self = Self::new(1.0, 0.0);
//             } else {
//                 *self = Self::new(-1.0, 0.0);
//             }
//             return Some(())
//         }
//
//         else if self.mixed_base == rhs.mixed_base &&
//             let Some(p1) = self.real_mixed_pow() &&
//             let Some(p2) = rhs.real_mixed_pow() &&
//             let Some(b) = self.real_mixed_base() && b != 0.0
//         {
//             if let Some(b_mut) = self.real_mixed_base_mut() {
//                 b_mut.real = b.mixed_powf(p1 - p2)
//             }
//             return Some(())
//         }
//
//         else if (self.mixed_base == rhs.mixed_base && self.mixed_pow == rhs.mixed_pow && self.is_mixed_pow_len_big()) ||
//             (self.real_mixed_pow().is_none() && self.mixed_pow.is_some() && !rhs.is_im_value())
//         {
//             if let Some(e) = &mut self.mixed_mul {
//                 e.div(rhs)?;
//                 return Some(())
//             }
//         }
//
//         else if self.mixed_base == rhs.mixed_base {
//             if let Some(e) = &mut self.mixed_pow {
//                 e.sub(rhs);
//                 return Some(())
//             }
//             if self.is_mixed_pow_zero() {
//                 *self = Self::new(1.0, 0.0);
//                 return Some(())
//             }
//         }
//
//         let div = |e1: &ImNumber, e2: &ImNumber| -> Option<ImNumber> {
//             if e2.real == 0.0 { return None }
//             else if e1.is_equal_by_abs(e2) {
//                 return Some(ImNumber::new(e1.real / e2.real, 0.0))
//             } else if e1.is_real() {
//                 return Some(ImNumber::new(e1.real / e2.real, -e2.im_mixed_pow))
//             } else if e2.is_real() {
//                 return Some(ImNumber::new(e1.real / e2.real, e1.im_mixed_pow))
//             } else if !e1.is_real() && !e2.is_real() {
//                 return Some(ImNumber::new(e1.real / e2.real, 0.0))
//             }
//             None
//         };
//
//         let mut elems_mut = Vec::<ImNumber>::new();
//
//         for e1 in self.mixed_base.iter() {
//             for e2 in rhs.mixed_base.iter() {
//                 if let Some(e) = div(e1, e2) {
//                     if e.real != 0.0 { elems_mut.push(e)}
//                 }
//                 else if self.mixed_base.len() == 1 && rhs.mixed_base.len() == 1 {
//                     if e2.real == 0.0 { return None }
//                     let mut e = Box::<ImNumber>::default();
//                     if e1.is_real() { *e = *e2 } else { *e = *e1 }
//                     e.real = e1.real / e2.real;
//                     if e.real != 0.0 { elems_mut.push(*e)}
//                 }
//                 else if self.mixed_base.len() > 1 && rhs.mixed_base.len() == 1 && e1.real != 0.0 {
//                     elems_mut.push(*e1)
//                 }
//             }
//         }
//
//         if elems_mut.is_empty() { elems_mut.push(ImNumber { real: 0.0, im_mixed_pow: 0.0 }) }
//         self.mixed_base = elems_mut;
//         self.collect();
//         Some(())
//     }
//
// }