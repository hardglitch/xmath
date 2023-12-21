use std::ptr::swap;
use crate::im_numbers::im_number::ImNumber;
use crate::utils::{AdvancedEQ, default};

#[derive(Debug, Clone)]
pub(crate) struct ImExpression {
    pub(crate) base: Vec<ImNumber>,
    pub(crate) pow: Vec<ImNumber>,
    pub(crate) mul: Vec<ImNumber>,
}

impl PartialEq for ImExpression {
    fn eq(&self, other: &Self) -> bool {
        self.pow == other.pow && self.base == other.base && self.mul == other.mul
    }
}

impl Default for ImExpression {
    fn default() -> Self {
        let expr_one = ImNumber::new(1.0, 0.0);
        Self {
            base: vec![],
            pow: vec![expr_one],
            mul: vec![expr_one],
        }
    }
}

impl ImExpression {

    pub(crate) fn is_equal_by_abs(&self, other: &Self) -> bool {
        if let Some(b1) = self.real_base() &&
           let Some(b2) = other.real_base()
        {
            return b1.abs() == b2.abs() && self.pow == other.pow && self.mul == other.mul
        }
        else if let Some(m1) = self.real_mul() &&
                let Some(m2) = other.real_mul()
        {
            return m1.abs() == m2.abs() && self.pow == other.pow && self.base == other.base
        }
        false
    }

    pub(crate) fn is_base_zero(&self) -> bool {
        if let Some(e) = self.real_base() && e == 0.0 { true } else { false }
    }

    pub(crate) fn is_mul_zero(&self) -> bool {
        if let Some(e) = self.real_mul() && e == 0.0 { true } else { false }
    }

    pub(crate) fn is_pow_zero(&self) -> bool {
        if let Some(n) = self.real_pow() && n == 0.0 { true } else { false }
    }

    pub(crate) fn is_mul_addable(&self, other: &Self) -> bool {
        self.base == other.base &&
            self.pow == other.pow &&
            self.real_pow().is_none()
    }

    pub(crate) fn real_pow(&self) -> Option<f64> {
        if let Some(p) = self.pow.first() && self.pow.len() == 1 && p.is_real() {
            return Some(p.real)
        }
        None
    }

    pub(crate) fn real_pow_mut(&mut self) -> Option<&mut ImNumber> {
        if self.pow.len() == 1 && let Some(p) = self.pow.first_mut() && p.is_real() {
            return Some(p)
        }
        None
    }

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
        if let Some(m) = self.mul.first() && self.mul.len() == 1 && m.is_real() {
            return Some(m.real)
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn im_base(&self) -> Option<f64> {
        if let Some(p) = self.base.first() && self.base.len() == 1 && !p.is_real() {
            return Some(p.im_pow)
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn real_mul_mut(&mut self) -> Option<&mut ImNumber> {
        if self.mul.len() == 1 && let Some(m) = self.mul.first_mut() &&  m.is_real() {
            return Some(m)
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn clean(&mut self) {
        while let Some((i, _)) = self.base.iter()
            .enumerate()
            .find(|(_, n)| n.real == 0.0)
        {
            self.base.remove(i);
        }
    }

    pub(crate) fn zero_fixer(&mut self) {
        if let Some(p) = self.real_pow_mut() && p.real == 0.0 { p.im_pow = 1.0 }
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

    pub(crate) unsafe fn add(&mut self, rhs: &mut Self) {
        if self.base.len() < rhs.base.len() {
            swap(self, rhs)
        }

        self.base.iter_mut().for_each(|e| e.pair_checker());
        rhs.base.iter_mut().for_each(|e| e.pair_checker());
        self.zero_fixer();
        rhs.zero_fixer();

        if self.is_mul_addable(rhs) {
            let mut lhs_expr = ImExpression::default();
            lhs_expr.base.append(&mut self.mul);

            let mut rhs_expr = ImExpression::default();
            rhs_expr.base.append(&mut rhs.mul);

            lhs_expr.add(&mut rhs_expr);
            self.mul = lhs_expr.mul;
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
            self.base.iter_mut().for_each(|e| e.real = -e.real);
            rhs.base.iter_mut().for_each(|e| e.real = -e.real);
        }

        self.base.iter_mut().for_each(|e| e.pair_checker());
        rhs.base.iter_mut().for_each(|e| e.pair_checker());
        self.zero_fixer();
        rhs.zero_fixer();

        if self.is_mul_addable(rhs) {
            let mut lhs_expr = ImExpression::default();
            lhs_expr.base.append(&mut self.mul);

            let mut rhs_expr = ImExpression::default();
            rhs_expr.base.append(&mut rhs.mul);

            lhs_expr.sub(&mut rhs_expr);
            self.mul = lhs_expr.mul;
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

        if self.base.len() < rhs.base.len() {
            swap(self, rhs)
        }

        self.base.iter_mut().for_each(|e| e.pair_checker());
        rhs.base.iter_mut().for_each(|e| e.pair_checker());
        self.zero_fixer();
        rhs.zero_fixer();

        if self.base == rhs.base &&
            let Some(p1) = self.real_pow() &&
            let Some(p2) = rhs.real_pow() &&
            let Some(b) = self.real_base()
        {
            if let Some(b_mut) = self.real_base_mut() {
                b_mut.real = b.powf(p1 + p2)
            }
            return
        }

        else if self.base == rhs.base {
            if self.mul.len() > 1 || self.real_mul().is_none() {
                let mut lhs_mul = ImExpression::default();
                lhs_mul.base.append(&mut self.mul);

                let mut rhs_mul = ImExpression::default();
                rhs_mul.base.append(&mut rhs.mul);

                lhs_mul.mul(&mut rhs_mul);
                self.mul = lhs_mul.base;
                if self.is_mul_zero() { return }
            }

            if self.pow.len() > 1 || self.real_pow().is_none() {
                let mut lhs_pow = ImExpression::default();
                lhs_pow.base.append(&mut self.pow);

                let mut rhs_pow = ImExpression::default();
                rhs_pow.base.append(&mut rhs.pow);

                lhs_pow.add(&mut rhs_pow);
                self.pow = lhs_pow.base;
                if self.is_pow_zero() {
                    self.mul = vec![ImNumber::new(1.0, 0.0)];
                    self.pow = vec![ImNumber::new(1.0, 0.0)];
                    self.base = vec![ImNumber::new(1.0, 0.0)];
                    return
                }
            }
            return
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
        if rhs.is_base_zero() { return None }

        if self.base.len() < rhs.base.len() {
            swap(self, rhs);
            self.pow.iter_mut().for_each(|e| e.real = -e.real);
            rhs.pow.iter_mut().for_each(|e| e.real = -e.real);
        }

        self.base.iter_mut().for_each(|e| e.pair_checker());
        rhs.base.iter_mut().for_each(|e| e.pair_checker());
        self.zero_fixer();
        rhs.zero_fixer();

        if self.base == rhs.base &&
            let Some(p1) = self.real_pow() &&
            let Some(p2) = rhs.real_pow() &&
            let Some(b) = self.real_base() && b != 0.0
        {
            if let Some(b_mut) = self.real_base_mut() {
                b_mut.real = b.powf(p1 - p2)
            }
            return Some(())
        }

        else if self.is_equal_by_abs(rhs) {
            self.mul = vec![ImNumber::new(1.0, 0.0)];
            self.pow = vec![ImNumber::new(1.0, 0.0)];
            self.base = vec![ImNumber::new(1.0, 0.0)];
            return Some(())
        }

        else if self.base == rhs.base {
            if self.mul.len() > 1 || self.real_mul().is_none() {
                let mut lhs_mul = ImExpression::default();
                lhs_mul.base.append(&mut self.mul);

                let mut rhs_mul = ImExpression::default();
                rhs_mul.base.append(&mut rhs.mul);

                lhs_mul.div(&mut rhs_mul)?;
                self.mul = lhs_mul.base;
                if self.is_mul_zero() { return Some(()) }
            }

            if self.pow.len() > 1 || self.real_pow().is_none() {
                let mut lhs_pow = ImExpression::default();
                lhs_pow.base.append(&mut self.pow);

                let mut rhs_pow = ImExpression::default();
                rhs_pow.base.append(&mut rhs.pow);

                lhs_pow.sub(&mut rhs_pow);
                self.pow = lhs_pow.base;
                if self.is_pow_zero() {
                    self.mul = vec![ImNumber::new(1.0, 0.0)];
                    self.pow = vec![ImNumber::new(1.0, 0.0)];
                    self.base = vec![ImNumber::new(1.0, 0.0)];
                    return Some(())
                }
            }
            return Some(())
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
