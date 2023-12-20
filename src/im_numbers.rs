use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use std::ptr::swap;
pub use ImOutput as Im;
use crate::utils::{AdvancedEQ, default};


pub trait XValue<T> {
    fn cast(self) -> T;
}
impl XValue<Im> for f64 {
    fn cast(self) -> Im {
        let im_num = ImNumber::new(self, 0.0);
        let expr_one = ImNumber::new(1.0, 0.0);
        let im_expr = ImExpression { elems: vec![im_num], pow: vec![expr_one.clone()], mul: vec![expr_one] };
        ImOutput { exprs: vec![im_expr] }
    }
}


#[derive(Debug, Clone, Default)]
pub(crate) struct ImNumber {
    pub(crate) real: f64,
    pub(crate) im_pow: f64,
}
impl PartialEq for ImNumber {
    fn eq(&self, other: &Self) -> bool {
        self.real.is_equal(other.real, default::PRECISION) &&
        self.im_pow.is_equal(other.im_pow, default::PRECISION)
    }
}
impl ImNumber {
    fn new(real: f64, im_pow: f64) -> Self {
        Self { real, im_pow }
    }

    pub(crate) fn pair_checker(&mut self) {
        let pairs = (self.im_pow / 2.0).trunc();
        if pairs != 0.0 && pairs % 2.0 != 0.0 { self.real = -self.real }
        self.im_pow -= 2.0 * pairs;
    }

    fn is_real(&self) -> bool {
        self.im_pow == 0.0
    }
}


#[derive(Debug, Clone)]
pub(crate) struct ImExpression {
    pub(crate) elems: Vec<ImNumber>,
    pub(crate) pow: Vec<ImNumber>,
    pub(crate) mul: Vec<ImNumber>,
}
impl PartialEq for ImExpression {
    fn eq(&self, other: &Self) -> bool {
        self.pow == other.pow && self.elems == other.elems && self.mul == other.mul
    }
}
impl Default for ImExpression {
    fn default() -> Self {
        let expr_one = ImNumber::new(1.0, 0.0);
        Self {
            elems: vec![],
            pow: vec![expr_one.clone()],
            mul: vec![expr_one],
        }
    }
}
impl ImExpression {
    fn new(i: &str) -> Option<Self> {
        let im_res = i
            .trim()
            .to_lowercase()
            .split('i')
            .enumerate()
            .filter_map(|(i, e)| if i == 0 { Some(e) } else { None })
            .collect::<Vec<&str>>()
            .first()
            .map(|s| s.parse::<f64>());

        if let Some(Ok(num)) = im_res {
            let im_num = ImNumber::new(num, 1.0);
            let expr_one = ImNumber::new(1.0, 0.0);
            let expr = Self {
                elems: vec![im_num],
                pow: vec![expr_one.clone()],
                mul: vec![expr_one]
            };
            return Some(expr)
        }
        None
    }

    // pub(crate) fn clean(&mut self) {
    //     while let Some((i, _)) = self.elems.iter()
    //         .enumerate()
    //         .find(|(_, n)| n.real == 0.0)
    //     {
    //         self.elems.remove(i);
    //     }
    // }

    pub(crate) fn is_zero(&self) -> bool {
        return if let Some(e) = self.mul.first() &&
            self.mul.len() == 1 && e.real == 0.0 { true } else { false }
    }

    pub(crate) fn is_one(&self) -> bool {
        return if let Some(e) =
            self.pow.first() &&
            self.pow.len() == 1 &&
            e.real == 0.0 &&
            !self.is_zero()
        { true } else { false }
    }

    // pub(crate) fn is_powable(&self) -> bool {
    //     return if let Some(p) = self.pow.first() &&
    //         self.pow.len() == 1 && p.is_real() &&
    //         let Some(b) = self.elems.first() &&
    //         self.elems.len() == 1 && b.is_real()
    //     { true } else { false }
    // }

    fn is_addable(&self, other: &Self) -> bool {
         self.elems == other.elems && self.pow == other.pow &&
            self.elems.len() > 1 && self.pow.len() > 1
    }

    fn real_pow(&self) -> Option<f64> {
        if let Some(p) = self.pow.first() && self.pow.len() == 1 && p.is_real() {
            return Some(p.real)
        }
        None
    }

    // fn real_pow_mut(&mut self) -> Option<&mut ImNumber> {
    //     if self.pow.len() == 1 && let Some(p) = self.pow.first_mut() && p.is_real() {
    //         return Some(p)
    //     }
    //     None
    // }

    fn real_base(&mut self) -> Option<f64> {
        if let Some(b) = self.elems.first() && self.elems.len() == 1 && b.is_real() {
            return Some(b.real)
        }
        None
    }

    fn real_base_mut(&mut self) -> Option<&mut ImNumber> {
        if self.elems.len() == 1 && let Some(b) = self.elems.first_mut() && b.is_real() {
            return Some(b)
        }
        None
    }

    fn real_mul(&self) -> Option<f64> {
        if let Some(m) = self.pow.first() && self.pow.len() == 1 && m.is_real() {
            return Some(m.real)
        }
        None
    }

    unsafe fn add(&self, rhs: &Self) -> Self {

        let lhs = &mut self.clone();
        let rhs = &mut rhs.clone();
        if lhs.elems.len() < rhs.elems.len() {
            swap(lhs, rhs)
        }

        lhs.elems.iter_mut().for_each(|e| e.pair_checker());
        rhs.elems.iter_mut().for_each(|e| e.pair_checker());

        if lhs.is_addable(rhs) {
            let mut lhs_expr = Box::<ImExpression>::default();
            lhs_expr.elems = lhs.mul.clone();

            let mut rhs_expr = Box::<ImExpression>::default();
            rhs_expr.elems = rhs.mul.clone();

            lhs.mul = lhs_expr.add(&rhs_expr).elems;
        }

        let eq_plus = |e1: &ImNumber, e2: &ImNumber| -> Option<ImNumber> {
           if e1.im_pow.is_equal(e2.im_pow, default::PRECISION) {
               return Some(ImNumber::new(e1.real + e2.real, e1.im_pow))
           }
           None
        };

        let mut expr = lhs.clone();
        expr.elems.clear();

        for e1 in lhs.elems.iter() {
            for e2 in rhs.elems.iter() {
                if let Some(e) = eq_plus(e1, e2) {
                    expr.elems.push(e)
                } else if lhs.elems.len() == 1 && rhs.elems.len() == 1 {
                    expr.elems.push(e1.clone());
                    expr.elems.push(e2.clone());
                } else if lhs.elems.len() > 1 && rhs.elems.len() == 1 {
                    expr.elems.push(e1.clone());
                }
            }
        }

        // expr.clean();
        expr
    }

    unsafe fn sub(&self, rhs: &Self) -> Self {

        let lhs = &mut self.clone();
        let rhs = &mut rhs.clone();
        if lhs.elems.len() < rhs.elems.len() {
            swap(lhs, rhs);
            lhs.elems.iter_mut().for_each(|e| e.real = -e.real);
            rhs.elems.iter_mut().for_each(|e| e.real = -e.real);
        }

        lhs.elems.iter_mut().for_each(|e| e.pair_checker());
        rhs.elems.iter_mut().for_each(|e| e.pair_checker());

        if lhs.is_addable(rhs) {
            let mut lhs_expr = Box::<ImExpression>::default();
            lhs_expr.elems = lhs.mul.clone();

            let mut rhs_expr = Box::<ImExpression>::default();
            rhs_expr.elems = rhs.mul.clone();

            lhs.mul = lhs_expr.sub(&rhs_expr).elems;
        }

        let eq_sub = |e1: &ImNumber, e2: &ImNumber| -> Option<ImNumber> {
            if e1.im_pow.is_equal(e2.im_pow, default::PRECISION) {
                return Some(ImNumber::new(e1.real - e2.real, e1.im_pow))
            }
            None
        };

        let mut expr = lhs.clone();
        expr.elems.clear();

        for e1 in lhs.elems.iter() {
            for e2 in rhs.elems.iter() {
                if let Some(e) = eq_sub(e1, e2) {
                    expr.elems.push(e)
                } else if lhs.elems.len() == 1 && rhs.elems.len() == 1 {
                    expr.elems.push(e1.clone());

                    let mut e = Box::<ImNumber>::default();
                    *e = e2.clone();
                    e.real = -e.real;
                    expr.elems.push(*e);
                } else if lhs.elems.len() > 1 && rhs.elems.len() == 1 {
                    expr.elems.push(e1.clone());
                }
            }
        }

        // expr.clean();
        expr
    }

    unsafe fn mul(&mut self, rhs: &mut Self) {

        if self.elems.len() < rhs.elems.len() {
            swap(self, rhs)
        }

        self.elems.iter_mut().for_each(|e| e.pair_checker());
        rhs.elems.iter_mut().for_each(|e| e.pair_checker());

        if self.elems == rhs.elems &&
            let Some(p1) = self.real_pow() &&
            let Some(p2) = rhs.real_pow() &&
            let Some(b) = self.real_base()
        {
            if let Some(b_mut) = self.real_base_mut() {
                b_mut.real = b.powf(p1 + p2)
            }
            return
        }

        else if self.elems == rhs.elems {
            if self.mul.len() > 1 || self.real_mul().is_none() {
                let mut lhs_mul = ImExpression::default();
                lhs_mul.elems = self.mul.clone();
                let mut rhs_mul = ImExpression::default();
                rhs_mul.elems.append(rhs.mul.as_mut());
                lhs_mul.mul(&mut rhs_mul);
                self.mul = lhs_mul.mul;
                if self.is_zero() { return }
            }

            if self.pow.len() > 1 || self.real_pow().is_none() {
                let mut lhs_pow = Box::<ImExpression>::default();
                lhs_pow.elems = self.pow.clone();
                let mut rhs_pow = Box::<ImExpression>::default();
                rhs_pow.elems = rhs.pow.clone();
                self.pow = lhs_pow.add(&rhs_pow).elems;
                if self.is_one() {
                    self.mul = vec![ImNumber::new(1.0, 0.0)];
                    self.pow = vec![ImNumber::new(1.0, 0.0)];
                    self.elems = vec![ImNumber::new(1.0, 0.0)];
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

        for e1 in self.elems.iter() {
            for e2 in rhs.elems.iter() {
                if let Some(e) = mul(e1, e2) {
                    elems_mut.push(e)
                } else if self.elems.len() == 1 && rhs.elems.len() == 1 {
                    let mut e = Box::<ImNumber>::default();
                    if e1.is_real() { *e = e2.clone() } else { *e = e1.clone() }
                    e.real = e2.real * e1.real;
                    elems_mut.push(*e);
                } else if self.elems.len() > 1 && rhs.elems.len() == 1 {
                    elems_mut.push(e1.clone());
                }
            }
        }

        self.elems = elems_mut;
    }

    unsafe fn div(&self, rhs: &ImExpression) -> ImExpression {

        let lhs = &mut self.clone();
        let rhs = &mut rhs.clone();

        if lhs.elems.len() < rhs.elems.len() {
            swap(lhs, rhs);
            lhs.pow.iter_mut().for_each(|e| e.real = -e.real);
            rhs.pow.iter_mut().for_each(|e| e.real = -e.real);
        }

        // rhs.clean();
        lhs.elems.iter_mut().for_each(|e| e.pair_checker());
        rhs.elems.iter_mut().for_each(|e| e.pair_checker());

        if lhs.is_addable(rhs) {
            let mut lhs_expr = Box::<ImExpression>::default();
            lhs_expr.elems = lhs.mul.clone();

            let mut rhs_expr = Box::<ImExpression>::default();
            rhs_expr.elems = rhs.mul.clone();

            lhs.mul = lhs_expr.div(&rhs_expr).elems;
        }

        let div = |e1: &ImNumber, e2: &ImNumber| -> Option<ImNumber> {
            if e1 == e2 {
                return Some(ImNumber::new(1.0, 0.0))
            } else if e1.is_real() {
                return Some(ImNumber::new(e1.real / e2.real, -e2.im_pow))
            } else if e2.is_real() {
                return Some(ImNumber::new(e1.real / e2.real, e1.im_pow))
            }
            None
        };

        let mut expr = lhs.clone();
        expr.elems.clear();

        for e1 in lhs.elems.iter() {
            for e2 in rhs.elems.iter() {
                if let Some(e) = div(e1, e2) {
                    expr.elems.push(e)
                } else if lhs.elems.len() == 1 && rhs.elems.len() == 1 {
                    let mut e = Box::<ImNumber>::default();
                    if e1.is_real() { *e = e2.clone() } else { *e = e1.clone() }
                    e.real = e2.real / e1.real;
                    expr.elems.push(*e);
                } else if lhs.elems.len() > 1 && rhs.elems.len() == 1 {
                    expr.elems.push(e1.clone());
                }
            }
        }

        // expr.clean();
        expr
    }
}


#[derive(Debug, Clone, Default)]
pub struct ImOutput {
    pub(crate) exprs: Vec<ImExpression>,
}
fn format_im_number(num: &[ImNumber]) -> String {
    num.iter()
        .map(|n| {
            if n.im_pow.is_equal(1.0, default::PRECISION) {
                format!("{}i", n.real)
            } else if n.im_pow.is_equal(0.0, default::PRECISION) {
                format!("{}", n.real)
            } else {
                format!("{}i^{}", n.real, n.im_pow)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
fn format_im_expr(expr: &[ImExpression]) -> String {
    expr.iter()
        .map(|e| {
            let mut pow = "".to_string();
            if !e.pow.first().unwrap().real.is_equal(1.0, default::PRECISION) {
                pow = format_im_number(&e.pow)
            }

            let mut mul = "".to_string();
            if !e.mul.first().unwrap().real.is_equal(1.0, default::PRECISION) {
                mul = format_im_number(&e.mul)
            }

            format!("{}{}{}", mul, format_im_number(&e.elems), pow)
        })
        .collect::<Vec<String>>()
        .join(" ")
}
impl Display for ImOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_im_expr(&self.exprs))
    }
}
impl PartialEq for ImOutput {
    fn eq(&self, other: &Self) -> bool {
        self.exprs == other.exprs
    }
}
impl Add for ImOutput {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self {
        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                if e1.pow == e2.pow {
                    let mut e = Box::<ImExpression>::default();
                    *e = e1.add(&e2);
                    exprs.push(*e);
                } else {
                    exprs.push(e1.clone());
                    exprs.push(e2.clone());
                }
            });
        }
        self.exprs.clear();
        self.exprs.append(&mut exprs);
        self.clean();
        self
    }
}
impl Sub for ImOutput {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self {
        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                if e1.pow == e2.pow {
                    let mut e = Box::<ImExpression>::default();
                    *e = e1.sub(&e2);
                    exprs.push(*e);
                } else {
                    exprs.push(e1.clone());
                    exprs.push(e2.clone());
                }
            });
        }
        self.exprs.clear();
        self.exprs.append(&mut exprs);
        self.clean();
        self
    }
}
impl Mul for ImOutput {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self {
        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop().as_mut() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                e1.mul(e2);
                if e1.is_one() {
                    let mut one = ImExpression::default();
                    one.elems.push(ImNumber::new(1.0, 0.0));
                    exprs.push(one);
                }
                else if !e1.elems.is_empty() && !e1.is_zero() {
                    exprs.push(e1.clone())
                }
            });
        }
        self.exprs = exprs;
        self.clean();
        self
    }
}
impl Div for ImOutput {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self {
        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                if e1.pow == e2.pow {
                    let mut e = Box::<ImExpression>::default();
                    *e = e1.div(&e2);
                    exprs.push(*e);
                } else {
                    exprs.push(e1.clone());
                    exprs.push(e2.clone());
                }
            });
        }
        self.exprs.clear();
        self.exprs.append(&mut exprs);
        self.clean();
        self
    }
}
impl ImOutput {
    pub fn new(i: &str) -> Result<Self, Box<dyn Error>> {
        if let Some(im_expr) = ImExpression::new(i) {
            return Ok(Self { exprs: vec![im_expr] })
        }
        Err("Invalid argument 'i'.".into())
    }
    pub(crate) fn clean(&mut self) {
        while let Some((i, _)) = self.exprs.iter()
            .enumerate()
            .find(|(_, n)|
                if let Some(m) = n.mul.first() && n.mul.len() > 1 {
                    m.real == 0.0
                } else {false} )
        {
            self.exprs.remove(i);
        }

        let mut c = 0;
        while c < self.exprs.len()
        {
            if let Some(e) = self.exprs.get_mut(c) {
                while let Some((i, _)) = e.elems.iter()
                    .enumerate()
                    .find(|(_, n)| n.real == 0.0)
                {
                    e.elems.remove(i);
                }
            }

            if let Some(e) = self.exprs.get(c) && e.elems.is_empty() {
                self.exprs.remove(c);
            }
            c += 1;
        }
    }
}
