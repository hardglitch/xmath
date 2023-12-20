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
        .join("")
}

pub(crate) fn format_im_expr(expr: &[ImExpression]) -> String {
    if expr.is_empty() { return "0".to_string() }

    expr.iter()
        .map(|e| {
            let mut pow = "".to_string();
            if !e.real_pow().is_some_and(|n| n == 1.0) {
                if e.pow.len() > 1 { pow = ["(", &format_im_number(&e.pow)[1..], ")"].join("") }
                pow = ["^", &pow].join("");
            }

            let mut mul = "".to_string();
            if !e.real_mul().is_some_and(|n| n == 1.0) && e.mul.len() > 1 {
                mul = ["(", &format_im_number(&e.mul)[1..], ")"].join("")
            }

            let mut base = "".to_string();
            if e.base.len() > 1 { base = ["(", &format_im_number(&e.base)[1..], ")"].join("") }

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

// impl Neg for ImOutput {
//     type Output = Self;
//
//     // This is only used to create variables in outer API.
//     fn neg(self) -> Self::Output {
//         let mut im_num= ImNumber::default();
//
//         if let Some(real) = self.exprs.first().unwrap().real_base() &&
//            let Some(im_pow) = self.exprs.first().unwrap().im_base()
//         {
//             im_num.real = real;
//             im_num.im_pow = im_pow;
//         }
//         let expr_one = ImNumber::new(1.0, 0.0);
//         let im_expr = ImExpression { base: vec![im_num], pow: vec![expr_one.clone()], mul: vec![expr_one] };
//         ImOutput { exprs: vec![im_expr] }
//     }
// }

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
        self.clean();
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
            if let Some(e) = self.exprs.get(c) && e.base.is_empty() {
                self.exprs.remove(c);
            }
            c += 1;
        }
    }

    pub fn is_zero(&self) -> bool {
        self.exprs.is_empty() || self.exprs.first().is_some_and(|e| e.is_base_zero())
    }
}
