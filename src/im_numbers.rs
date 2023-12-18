use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul, Sub};
use std::ptr::swap;
pub use ImOutput as Im;
use crate::utils::{AdvancedEQ, default};


pub trait XValue<T> {
    fn cast(self) -> T;
}
impl XValue<Im> for f64 {
    fn cast(self) -> Im {
        let im_num = ImNumber::new(self, 0.0);
        let expr_pow = ImNumber::new(1.0, 0.0);
        let im_expr = ImExpression { elems: vec![im_num], pow: vec![expr_pow], mul: 1.0 };
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

    fn pair_checker(&mut self) {
        let pair = (self.im_pow / 2.0).trunc();
        if pair.abs() > 0.0 { self.real *= (-1.0) * pair }
        self.im_pow -= 2.0 * pair;
    }

    fn is_real(&self) -> bool {
        self.im_pow == 0.0
    }
}


#[derive(Debug, Clone)]
pub(crate) struct ImExpression {
    pub(crate) elems: Vec<ImNumber>,
    pub(crate) pow: Vec<ImNumber>,
    pub(crate) mul: f64,
}
impl PartialEq for ImExpression {
    fn eq(&self, other: &Self) -> bool {
        self.pow == other.pow && self.elems == other.elems && self.mul == other.mul
    }
}
impl Default for ImExpression {
    fn default() -> Self {
        let expr_pow = ImNumber::new(1.0, 0.0);
        Self {
            elems: vec![],
            pow: vec![expr_pow],
            mul: 1.0,
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
            let expr_pow = ImNumber::new(1.0, 0.0);
            let expr = Self { elems: vec![im_num], pow: vec![expr_pow], mul: 1.0 };
            return Some(expr)
        }
        None
    }

    unsafe fn add(&self, rhs: &ImExpression) -> ImExpression {

        let lhs = &mut self.clone();
        let rhs = &mut rhs.clone();
        if lhs.elems.len() < rhs.elems.len() {
            swap(lhs, rhs)
        }

        lhs.elems.iter_mut().for_each(|e| e.pair_checker());
        rhs.elems.iter_mut().for_each(|e| e.pair_checker());

        if lhs.elems == rhs.elems && lhs.pow == rhs.pow &&
           lhs.elems.len() > 1 && rhs.elems.len() > 1 { lhs.mul += rhs.mul }

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

        expr
    }

    unsafe fn sub(&self, rhs: &ImExpression) -> ImExpression {

        let lhs = &mut self.clone();
        let rhs = &mut rhs.clone();
        if lhs.elems.len() < rhs.elems.len() {
            swap(lhs, rhs);
            lhs.elems.iter_mut().for_each(|e| e.real *= -1.0);
            rhs.elems.iter_mut().for_each(|e| e.real *= -1.0);
        }

        lhs.elems.iter_mut().for_each(|e| e.pair_checker());
        rhs.elems.iter_mut().for_each(|e| e.pair_checker());

        if lhs.elems == rhs.elems && lhs.pow == rhs.pow { lhs.mul += rhs.mul }

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
                    if e1.is_real() { *e = e2.clone() } else { *e = e1.clone() }
                    e.real *= -1.0;
                    expr.elems.push(*e);
                } else if lhs.elems.len() > 1 && rhs.elems.len() == 1 {
                    expr.elems.push(e1.clone());
                }
            }
        }

        expr
    }

    unsafe fn mul(&self, rhs: &ImExpression) -> ImExpression {

        let lhs = &mut self.clone();
        let rhs = &mut rhs.clone();
        if lhs.elems.len() < rhs.elems.len() {
            swap(lhs, rhs)
        }

        lhs.elems.iter_mut().for_each(|e| e.pair_checker());
        rhs.elems.iter_mut().for_each(|e| e.pair_checker());

        if lhs.elems == rhs.elems && lhs.pow == rhs.pow { lhs.mul += rhs.mul }

        let eq_mul = |e1: &ImNumber, e2: &ImNumber| -> Option<ImNumber> {
            if e1.im_pow.is_equal(e2.im_pow, default::PRECISION) {
                return if e1.is_real() {
                    Some(ImNumber::new(e1.real * e2.real, 0.0))
                } else {
                    Some(ImNumber::new(-1.0, 0.0))
                }
            }
            None
        };

        let mut expr = lhs.clone();
        expr.elems.clear();

        for e1 in lhs.elems.iter() {
            for e2 in rhs.elems.iter() {
                if let Some(e) = eq_mul(e1, e2) {
                    expr.elems.push(e)
                } else if lhs.elems.len() == 1 && rhs.elems.len() == 1 {
                    let mut e = Box::<ImNumber>::default();
                    if e1.is_real() { *e = e2.clone() } else { *e = e1.clone() }
                    e.real = e2.real * e1.real;
                    expr.elems.push(*e);
                } else if lhs.elems.len() > 1 && rhs.elems.len() == 1 {
                    expr.elems.push(e1.clone());
                }
            }
        }

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
            if !e.mul.is_equal(1.0, default::PRECISION) {
                mul = e.mul.to_string()
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
        self
    }
}
impl Mul for ImOutput {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self {
        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                if e1.pow == e2.pow {
                    let mut e = Box::<ImExpression>::default();
                    *e = e1.mul(&e2);
                    exprs.push(*e);
                } else {
                    exprs.push(e1.clone());
                    exprs.push(e2.clone());
                }
            });
        }
        self.exprs.clear();
        self.exprs.append(&mut exprs);
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
}
