use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use crate::im_numbers::core::{ImExpression, Sign};
use crate::im_numbers::im_number::ImNumber;


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
            if let Some(n) = e.real_pow() && let Some(p) = &e.pow {
                if e.is_pow_len_big() {
                    *pow = ["(", &format_im_number(&p.base), ")"].concat()
                } else if n != 1.0 {
                    *pow = format_im_number(&p.base)
                }
                if n != 1.0 && n > 0.0 {
                    *pow = ["^", &pow].concat();
                } else if n < 0.0 {
                    *div = "/".to_string();
                    if n == -1.0 { *pow = "".to_string() }
                }
            }

            let base = &mut "".to_string();
            if e.is_base_len_big() {
                *base = ["(", &format_im_number(&e.base), ")"].concat();
                // swap(mul, div);
            } else {
                *base = format_im_number(&e.base)
            }

            let mul = &mut "".to_string();
            if let Some(m) = &e.mul {
                if e.is_mul_len_big() {
                    *mul = ["(", &format_im_number(&m.base), ")"].concat()
                } else if e.real_mul().is_some_and(|n| n != 1.0) {
                    *mul = format_im_number(&m.base)
                }
                if let Some(m) = &e.mul &&
                   let Some(p) = &m.pow &&
                    (p.base.first().is_some_and(|n| n.real < 0.0) ||
                     p.mul.as_ref().is_some_and(|e|
                         e.base.first().is_some_and(|n| n.real < 0.0))) {
                    *div = "/".to_string();
                    return format!("{}{}{}{}", base, pow, div, mul)
                }
            }

            format!("{}{}{}{}", mul, div, base, pow)
        })
        .collect::<Vec<String>>()
        .concat()
}

impl Display for Im {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_im_expr(&self.exprs))
    }
}

impl PartialEq for Im {
    fn eq(&self, other: &Self) -> bool {
        self.exprs == other.exprs
    }
}

impl Mul for Im {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self {
        if self.is_none() { return self }
        if rhs.is_none() { return rhs }

        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop().as_mut() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                e1.mul(e2);
                exprs.push(e1.clone())
            });
        }
        self.exprs = exprs;
        self
    }
}

impl Div for Im {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self {
        if self.is_none() { return self }
        if rhs.is_none() { return rhs }

        let mut exprs = Vec::<ImExpression>::new();
        while let Some(e2) = rhs.exprs.pop().as_mut() {
            self.exprs.iter_mut().for_each(|e1| unsafe {
                #[allow(irrefutable_let_patterns)]
                if let s = e2.is_equal_by_abs(e1) && s != Sign::None {
                    if s == Sign::Plus {
                        exprs.push(ImExpression::new(1.0, 0.0));
                    } else {
                        exprs.push(ImExpression::new(-1.0, 0.0));
                    }
                }
                else if e2.base.len() == 1 {
                    if e1.div(e2).is_none() { return exprs.clear() }
                    exprs.push(e1.clone())
                } else {
                    if let Some(p) = &mut e2.pow {
                        p.neg();
                    } else {
                        e2.pow = Some(Box::new(ImExpression::new(-1.0, 0.0)))
                    }
                    if let Some(m) = &mut e1.mul {
                        m.mul(e2)
                    } else {
                        e1.mul = Some(Box::new(e2.clone()))
                    }
                    exprs.push(e1.clone());
                }
            });
        }
        self.exprs = exprs;
        self
    }
}