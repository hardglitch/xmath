use std::fmt::{Debug, Display, Formatter};
use std::mem::swap;
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
            let num = &mut "".to_string();
            if !n.is_real() && n.real == -1.0 {
                *num = "-".to_string()
            } else if (!n.is_real() && n.real != 1.0) || n.is_real() {
                *num = n.real.to_string()
            }

            let mut i = "";
            if !n.is_real() { i = "i" }

            let mut plus = "";
            if n.real >= 0.0 { plus = "+" }

            let im_pow = &mut "".to_string();
            if !n.is_real() && n.im_pow < 0.0 {
                *im_pow = "/".to_string();
            }
            format!("{}{}{}{}", plus, num, im_pow, i)
        })
        .collect::<Vec<String>>()
        .concat()[1..]
        .to_string()
}

pub(crate) fn format_im_expr(expr: &[ImExpression]) -> String {
    if expr.is_empty() { return "None".to_string() }

    expr.iter()
        .map(|e| {
            let pow = &mut "".to_string();
            let div = &mut "".to_string();
            if let Some(n) = e.real_pow() {
                if e.pow.len() > 1 {
                    *pow = ["(", &format_im_number(&e.pow), ")"].concat()
                } else if n != 1.0 {
                    *pow = format_im_number(&e.pow)
                }
                if n != 1.0 && n > 0.0 {
                    *pow = ["^", &pow].concat();
                } else if n < 0.0 {
                    *div = "/".to_string();
                    if n == -1.0 { *pow = "".to_string() }
                }
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
                *base = ["(", &format_im_number(&e.base), ")"].concat();
                swap(mul, div);
            } else {
                *base = format_im_number(&e.base)
            }

            format!("{}{}{}{}", mul, div, base, pow)
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
        if self.is_none() { return self }
        if rhs.is_none() { return rhs }

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
        if self.is_none() { return self }
        if rhs.is_none() { return rhs }

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
        if self.is_none() { return self }
        if rhs.is_none() { return rhs }

        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop().as_mut() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                e1.mul(e2);
                if !e1.base.is_empty() && !e1.is_mul_zero() {
                    exprs.push(e1.clone())
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
        if self.is_none() { return self }
        if rhs.is_none() { return rhs }

        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop().as_mut() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                if e2.base.len() == 1 {
                    if e1.div(e2).is_none() {
                        return exprs.clear()
                    }
                    if !e1.base.is_empty() && !e1.is_pow_zero() {
                        exprs.push(e1.clone())
                    }
                } else if !e1.base.is_empty() {
                    exprs.push(e1.clone());
                    let mut e = e2.clone();
                    e.pow.iter_mut().for_each(|n| n.real = -n.real);
                    exprs.push(e);
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
    pub fn is_none(&self) -> bool {
        self.exprs.is_empty()
    }
}
