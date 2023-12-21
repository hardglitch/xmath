use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use crate::im_numbers::im_expression::ImExpression;
use crate::im_numbers::im_number::ImNumber;


#[derive(Debug, Clone, Default)]
pub struct ImOutput {
    pub(crate) exprs: Vec<ImExpression>,
}

fn format_im_number(num: &[ImNumber]) -> String {
    num.iter()
        .map(|n| {
            let mut i = "";
            if !n.is_real() { i = "i" }
            let mut plus = "";
            if n.real >= 0.0 { plus = "+" }
            format!("{}{}{}", plus, n.real, i)
        })
        .collect::<Vec<String>>()
        .concat()[1..]
        .to_string()
}

pub(crate) fn format_im_expr(expr: &[ImExpression]) -> String {
    if expr.is_empty() { return "0".to_string() }

    expr.iter()
        .map(|e| {
            let pow = &mut "".to_string();
            if !e.real_pow().is_some_and(|n| n == 1.0) {
                if e.pow.len() > 1 {
                    *pow = ["(", &format_im_number(&e.pow), ")"].concat()
                } else {
                    *pow = format_im_number(&e.pow)
                }
                *pow = ["^", &pow].concat();
            }

            let mul = &mut "".to_string();
            if !e.real_mul().is_some_and(|n| n == 1.0) {
                if e.mul.len() > 1 {
                    *mul = ["(", &format_im_number(&e.mul), ")"].concat()
                } else {
                    *mul = format_im_number(&e.mul)
                }
            }

            let base = &mut "".to_string();
            if e.base.len() > 1 {
                *base = ["(", &format_im_number(&e.base), ")"].concat()
            } else {
                *base = format_im_number(&e.base)
            }

            format!("{}{}{}", mul, base, pow)
        })
        .collect::<Vec<String>>()
        .join("")
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
        while let Some(e2) = rhs.exprs.pop().as_mut() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                if e1.pow == e2.pow {
                    e1.add(e2);
                    exprs.push(e1.clone());
                } else {
                    exprs.push(e1.clone());
                    exprs.push(e2.clone());
                }
            });
        }
        self.exprs = exprs;
        self
    }
}

impl Sub for ImOutput {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self {
        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop().as_mut() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                if e1.pow == e2.pow {
                    e1.sub(e2);
                    exprs.push(e1.clone());
                } else {
                    exprs.push(e1.clone());
                    exprs.push(e2.clone());
                }
            });
        }
        self.exprs = exprs;
        self
    }
}

impl Mul for ImOutput {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self {
        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop().as_mut() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                if e1.base == e2.base || (e1.base != e2.base && e1.pow == e2.pow) {
                    e1.mul(e2);
                    if !e1.base.is_empty() && !e1.is_mul_zero() {
                        exprs.push(e1.clone())
                    } else if !e1.base.is_empty() {
                        exprs.push(e1.clone());
                        exprs.push(e2.clone());
                    }
                }
            });
        }
        self.exprs = exprs;
        self
    }
}

impl Div for ImOutput {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self {
        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop().as_mut() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                if e1.base == e2.base || (e1.base != e2.base && e1.pow == e2.pow) {
                    e1.div(e2).expect("Divide by 0");
                    if !e1.base.is_empty() && !e1.is_pow_zero() {
                        exprs.push(e1.clone())
                    } else if !e1.base.is_empty() {
                        exprs.push(e1.clone());
                        exprs.push(e2.clone());
                    }
                }
            });
        }
        self.exprs = exprs;
        self
    }
}

impl ImOutput {
    pub fn is_zero(&self) -> bool {
        self.exprs.is_empty() || self.exprs.first().is_some_and(|e| e.is_base_zero())
    }
}
