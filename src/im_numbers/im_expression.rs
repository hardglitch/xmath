use std::ptr::swap;
use crate::im_numbers::im_number::ImNumber;
use crate::utils::{AdvancedEQ, default};


#[derive(PartialEq)]
pub(crate) enum Sign {
    Plus,
    Minus,
    None,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ImExpression {
    pub(crate) base: Vec<ImNumber>,
    pub(crate) pow: Option<Box<ImExpression>>,
    pub(crate) mul: Option<Box<ImExpression>>,
}

impl PartialEq for ImExpression {
    fn eq(&self, other: &Self) -> bool {
        self.pow == other.pow && self.base == other.base && self.mul == other.mul
    }
}

impl ImExpression {

    pub(crate) fn new(real: f64, im_pow: f64) -> Self {
        Self {
            base: vec![ImNumber::new(real, im_pow)],
            pow: Default::default(),
            mul: Default::default(),
        }
    }

    pub(crate) fn is_equal_by_abs(&self, other: &Self) -> Sign {
        let mut neg_self = self.clone();
        neg_self.neg();
        if self == other { Sign::Plus }
        else if &neg_self == other { Sign::Minus }
        else { Sign::None }
    }

    pub(crate) fn is_base_zero(&self) -> bool {
        self.real_base().is_some_and(|n| n == 0.0)
    }

    pub(crate) fn is_mul_zero(&self) -> bool {
        self.mul.as_ref().is_some_and(|e| e.real_base().is_some_and(|n| n == 0.0))
    }

    pub(crate) fn is_pow_zero(&self) -> bool {
        self.pow.as_ref().is_some_and(|e| e.real_base().is_some_and(|n| n == 0.0))
    }

    pub(crate) fn is_mul_incrementable(&self, other: &Self) -> bool {
        self.base == other.base && self.pow == other.pow && self.pow.is_some()
    }

    pub(crate) fn real_pow(&self) -> Option<f64> {
        if let Some(ref pow) = self.pow &&
           let Some(p) = pow.base.first() &&
           self.base.len() == 1 && p.is_real()
        {
            return Some(p.real)
        }
        None
    }

    // pub(crate) fn real_pow_mut(&mut self) -> Option<&mut ImNumber> {
    //     if let Some(p) = self.pow.real_base_mut() {
    //         return Some(p)
    //     }
    //     None
    // }

    pub(crate) fn real_base(&self) -> Option<f64> {
        if let Some(b) = self.base.first() && self.base.len() == 1 && b.is_real() {
            return Some(b.real)
        }
        None
    }

    pub(crate) fn real_base_mut(&mut self) -> Option<&mut ImNumber> {
        if self.base.len() == 1 && let Some(b) = self.base.first_mut() && b.is_real() {
            return Some(b)
        }
        None
    }

    pub(crate) fn real_mul(&self) -> Option<f64> {
        if let Some(ref mul) = self.mul &&
            let Some(m) = mul.base.first() &&
            self.base.len() == 1 && m.is_real()
        {
            return Some(m.real)
        }
        None
    }

    // pub(crate) fn real_mul_mut(&mut self) -> Option<&mut ImNumber> {
    //     if let Some(m) = self.mul.real_base_mut() {
    //         return Some(m)
    //     }
    //     None
    // }

    // pub(crate) fn im_base(&self) -> Option<f64> {
    //     if let Some(p) = self.base.first() && self.base.len() == 1 && !p.is_real() {
    //         return Some(p.im_pow)
    //     }
    //     None
    // }

    pub(crate) fn is_im_value(&self) -> bool {
        for n in self.base.iter() {
            if n.im_pow > 0.0 { return true }
        }
        if self.pow.as_ref().is_some_and(|e| e.is_im_value()) ||
           self.mul.as_ref().is_some_and(|e| e.is_im_value())
        { return true }
        false
    }

    pub(crate) fn neg(&mut self) {
        if self.pow.is_none() && self.mul.is_none() {
            self.base.iter_mut().for_each(|e| e.real = -e.real);
        }
        else if self.pow.is_some() && self.mul.is_none() {
            self.mul = Some(Box::new(Self::new(-1.0, 0.0)))
        }
        else if self.pow.is_some() && self.mul.is_some() {
            self.mul.iter_mut().for_each(|e| e.neg())
        }
    }

    pub(crate) fn is_pow_len_big(&self) -> bool {
        self.pow.as_ref().is_some_and(|e| e.base.len() > 1)
    }

    pub(crate) fn is_mul_len_big(&self) -> bool {
        self.mul.as_ref().is_some_and(|e| e.base.len() > 1)
    }

    pub(crate) fn is_base_len_big(&self) -> bool {
        self.base.len() > 1
    }

    pub(crate) fn collect(&mut self) {
        if self.is_base_zero() || self.base.len() <= 1 { return; }

        let mut reals = ImNumber { real: 0.0, im_pow: 0.0 };
        let mut ims = ImNumber { real: 0.0, im_pow: 1.0 };
        let mut ims_div = ImNumber { real: 0.0, im_pow: -1.0 };

        self.base.iter().for_each(|n| {
            if n.is_real() {
                reals.real += n.real
            } else if n.im_pow == ims.im_pow {
                ims.real += n.real
            } else if n.im_pow == ims_div.im_pow {
                ims_div.real += n.real
            }
        });

        let mut list = vec![];
        if ims_div.real != 0.0 { list.push(ims_div) }
        if ims.real != 0.0 { list.push(ims) }
        list.push(reals);

        list.sort();
        list.reverse();
        self.base = list.to_vec();
    }

    fn mul_zero_check(&mut self, other: &mut Self) -> bool {
        if other.is_base_zero() || other.is_mul_zero()
        {
            *self = Self::new(0.0, 0.0);
            return true
        }
        else if self.is_base_zero() || self.is_mul_zero() {
            return true
        }
        false
    }

    pub(crate) unsafe fn add(&mut self, rhs: &mut Self) {
        if self.base.len() < rhs.base.len() {
            swap(self, rhs)
        }

        self.base.iter_mut().for_each(|e| e.pair_checker());
        rhs.base.iter_mut().for_each(|e| e.pair_checker());

        if self.is_mul_incrementable(rhs) &&
           let Some(ref mut m1) = self.mul &&
           let Some(ref mut m2) = rhs.mul
        {
            m1.add(m2)
        }

        let eq_plus = |e1: &ImNumber, e2: &ImNumber| -> Option<ImNumber> {
            if e1.im_pow.is_equal(e2.im_pow, default::PRECISION) {
                return Some(ImNumber::new(e1.real + e2.real, e1.im_pow))
            }
            None
        };

        let mut elems_mut = Vec::<ImNumber>::new();

        for e1 in self.base.iter() {
            for e2 in rhs.base.iter() {
                if let Some(e) = eq_plus(e1, e2) {
                    if e.real != 0.0 { elems_mut.push(e)}
                }
                else if self.base.len() == 1 && rhs.base.len() == 1 {
                    if e1.real != 0.0 { elems_mut.push(*e1)}
                    if e2.real != 0.0 { elems_mut.push(*e2)}
                }
                else if self.base.len() > 1 && rhs.base.len() == 1 && e1.real != 0.0 {
                    elems_mut.push(*e1)
                }
            }
        }

        if elems_mut.is_empty() { elems_mut.push(ImNumber { real: 0.0, im_pow: 0.0 }) }
        self.base = elems_mut;
    }

    pub(crate) unsafe fn sub(&mut self, rhs: &mut Self) {
        if self.base.len() < rhs.base.len() {
            swap(self, rhs);
            self.neg();
            rhs.neg();
        }

        self.base.iter_mut().for_each(|e| e.pair_checker());
        rhs.base.iter_mut().for_each(|e| e.pair_checker());

        if self.is_mul_incrementable(rhs) &&
            let Some(ref mut m1) = self.mul &&
            let Some(ref mut m2) = rhs.mul
        {
            m1.sub(m2)
        }

        let eq_sub = |e1: &ImNumber, e2: &ImNumber| -> Option<ImNumber> {
            if e1.im_pow.is_equal(e2.im_pow, default::PRECISION) {
                return Some(ImNumber::new(e1.real - e2.real, e1.im_pow))
            }
            None
        };

        let mut elems_mut = Vec::<ImNumber>::new();

        for e1 in self.base.iter() {
            for e2 in rhs.base.iter() {
                if let Some(e) = eq_sub(e1, e2) {
                    if e.real != 0.0 { elems_mut.push(e)}
                }
                else if self.base.len() == 1 && rhs.base.len() == 1 {
                    if e1.real != 0.0 { elems_mut.push(*e1)}

                    let mut e = Box::<ImNumber>::default();
                    *e = *e2;
                    e.real = -e.real;
                    if e.real != 0.0 { elems_mut.push(*e)}
                }
                else if self.base.len() > 1 && rhs.base.len() == 1 && e1.real != 0.0 {
                    elems_mut.push(*e1)
                }
            }
        }

        if elems_mut.is_empty() { elems_mut.push(ImNumber { real: 0.0, im_pow: 0.0 }) }
        self.base = elems_mut;
    }

    pub(crate) unsafe fn mul(&mut self, rhs: &mut Self) {
        if self.mul_zero_check(rhs) { return }

        if (self.base.len() < rhs.base.len()) ||
           (self.real_pow().is_none() && self.pow.is_some() && (rhs.real_pow().is_some() || rhs.pow.is_none()))
        {
            swap(self, rhs)
        }

        self.base.iter_mut().for_each(|e| e.pair_checker());
        rhs.base.iter_mut().for_each(|e| e.pair_checker());

        if self.base == rhs.base &&
            self.pow.is_none() &&
            rhs.pow.is_none() &&
            let Some(p1) = self.real_pow() &&
            let Some(p2) = rhs.real_pow()
        {
            if let Some(b_mut) = self.real_base_mut() {
                b_mut.real = b_mut.real.powf(p1 + p2)
            }
            return
        }

        else if (self.base == rhs.base && self.pow == rhs.pow && self.is_pow_len_big()) ||
            (self.real_pow().is_none() && self.pow.is_some() && !rhs.is_im_value())
        {
            if let Some(e) = &mut self.mul {
                e.mul(rhs);
                return
            }
        }

        else if self.base == rhs.base {
            if let Some(e) = &mut self.pow {
                e.add(rhs);
                return
            }
            if self.is_pow_zero() {
                *self = Self::new(1.0, 0.0);
                return
            }
        }

        let mul = |e1: &ImNumber, e2: &ImNumber| -> Option<ImNumber> {
            if e1.is_real() {
                return Some(ImNumber::new(e1.real * e2.real, e2.im_pow))
            } else if e2.is_real() {
                return Some(ImNumber::new(e1.real * e2.real, e1.im_pow))
            } else if !e1.is_real() && !e2.is_real() {
                return Some(ImNumber::new(e1.real * e2.real * (-1.0), 0.0))
            }
            None
        };

        let mut elems_mut = Vec::<ImNumber>::new();

        for e1 in self.base.iter() {
            for e2 in rhs.base.iter() {
                if let Some(e) = mul(e1, e2) {
                    if e.real != 0.0 { elems_mut.push(e)}
                }
                else if self.base.len() == 1 && rhs.base.len() == 1 {
                    let mut e = Box::<ImNumber>::default();
                    if e1.is_real() { *e = *e2 } else { *e = *e1 }
                    e.real = e2.real * e1.real;
                    if e.real != 0.0 { elems_mut.push(*e)}
                }
                else if self.base.len() > 1 && rhs.base.len() == 1 && e1.real != 0.0 {
                    elems_mut.push(*e1)
                }
            }
        }

        if elems_mut.is_empty() { elems_mut.push(ImNumber { real: 0.0, im_pow: 0.0 }) }
        self.base = elems_mut;
        self.collect();
    }

    pub(crate) unsafe fn div(&mut self, rhs: &mut Self) -> Option<()> {
        if rhs.is_base_zero() || rhs.is_mul_zero() { return None }

        if self.base.len() < rhs.base.len()
        {
            swap(self, rhs);
            self.pow.iter_mut().for_each(|e| e.neg());
            self.mul.iter_mut().for_each(|e| e.neg());
            rhs.pow.iter_mut().for_each(|e| e.neg());
            rhs.mul.iter_mut().for_each(|e| e.neg());
        }

        self.base.iter_mut().for_each(|e| e.pair_checker());
        rhs.base.iter_mut().for_each(|e| e.pair_checker());

        #[allow(irrefutable_let_patterns)]
        if let s = self.is_equal_by_abs(rhs) && s != Sign::None {
            if s == Sign::Plus {
                *self = Self::new(1.0, 0.0);
            } else {
                *self = Self::new(-1.0, 0.0);
            }
            return Some(())
        }

        else if self.base == rhs.base &&
            let Some(p1) = self.real_pow() &&
            let Some(p2) = rhs.real_pow() &&
            let Some(b) = self.real_base() && b != 0.0
        {
            if let Some(b_mut) = self.real_base_mut() {
                b_mut.real = b.powf(p1 - p2)
            }
            return Some(())
        }

        else if (self.base == rhs.base && self.pow == rhs.pow && self.is_pow_len_big()) ||
            (self.real_pow().is_none() && self.pow.is_some() && !rhs.is_im_value())
        {
            if let Some(e) = &mut self.mul {
                e.div(rhs)?;
                return Some(())
            }
        }

        else if self.base == rhs.base {
            if let Some(e) = &mut self.pow {
                e.sub(rhs);
                return Some(())
            }
            if self.is_pow_zero() {
                *self = Self::new(1.0, 0.0);
                return Some(())
            }
        }

        let div = |e1: &ImNumber, e2: &ImNumber| -> Option<ImNumber> {
            if e2.real == 0.0 { return None }
            else if e1.is_equal_by_abs(e2) {
                return Some(ImNumber::new(e1.real / e2.real, 0.0))
            } else if e1.is_real() {
                return Some(ImNumber::new(e1.real / e2.real, -e2.im_pow))
            } else if e2.is_real() {
                return Some(ImNumber::new(e1.real / e2.real, e1.im_pow))
            } else if !e1.is_real() && !e2.is_real() {
                return Some(ImNumber::new(e1.real / e2.real, 0.0))
            }
            None
        };

        let mut elems_mut = Vec::<ImNumber>::new();

        for e1 in self.base.iter() {
            for e2 in rhs.base.iter() {
                if let Some(e) = div(e1, e2) {
                    if e.real != 0.0 { elems_mut.push(e)}
                }
                else if self.base.len() == 1 && rhs.base.len() == 1 {
                    if e2.real == 0.0 { return None }
                    let mut e = Box::<ImNumber>::default();
                    if e1.is_real() { *e = *e2 } else { *e = *e1 }
                    e.real = e1.real / e2.real;
                    if e.real != 0.0 { elems_mut.push(*e)}
                }
                else if self.base.len() > 1 && rhs.base.len() == 1 && e1.real != 0.0 {
                    elems_mut.push(*e1)
                }
            }
        }

        if elems_mut.is_empty() { elems_mut.push(ImNumber { real: 0.0, im_pow: 0.0 }) }
        self.base = elems_mut;
        self.collect();
        Some(())
    }
}
