use std::any::{Any, TypeId};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Deref, Div, Mul, Sub};


#[derive(Debug, Default, Clone)]
pub struct ImNumber {
    real: f64,
}
impl Add for ImNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { real: self.real + rhs.real }
    }
}
impl Sub for ImNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { real: self.real - rhs.real }
    }
}
impl Mul for ImNumber {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { real: self.real * rhs.real }
    }
}
impl Div for ImNumber {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { real: self.real / rhs.real }
    }
}
impl Display for ImNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}i", self.real)
    }
}
impl ImNumber {
    pub fn new(num: f64) -> Self {
        Self { real: num }
    }

    pub fn mul_num(&self, num: f64) -> Self {
        Self { real: self.real * num }
    }

    pub fn div_num(&self, num: f64) -> Self {
        Self { real: self.real / num }
    }
}
impl IToken for ImNumber {
    fn get_value(&self) -> Self where Self: Sized {
        self.clone()
    }
}
impl IToken for f64 {
    fn get_value(&self) -> Self where Self: Sized {
        *self
    }
}
impl IToken for String
{
    fn get_value(&self) -> Self where Self: Sized {
        self.to_string()
    }
}

trait IToken
{
    fn get_value(&self) -> Self where Self: Sized;
}

struct Token {
    raw: String,
    value: Box<dyn IToken>,
}
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Token")
         .field(&self.raw)
         .field(&self.value.deref().type_id() )
         .finish()
    }
}
impl Token
{
    fn new(value: &str) -> Self {
        Self {
            raw: value.to_string(),
            value: Self::define(value),
        }
    }

    fn define(value: &str) -> Box<dyn IToken> {
        if value.find('i').is_some() &&
            let Ok(e) = value[0..value.len() - 1].parse::<f64>() {
            return Box::new(ImNumber::new(e))
        }
        if let Ok(e) = value.parse::<f64>() {
            return Box::new(e)
        }
        Box::new(value.to_string())
    }
}
#[derive(Debug)]
pub struct ImExpr {
    values: Vec<Token>
}
impl ImExpr {
    pub fn new(s: &str) -> Self {
        let tokens = s
            .to_lowercase()
            .split(' ')
            .map(|e| e.trim())
            .map(Token::new)
            .collect();

        Self { values: tokens }
    }

    pub fn execute(&self) {
        for v in &self.values {
            if v.value.get_value().type_id() == TypeId::of::<dyn IToken>() {
                println!("IToken");
            }
        }
    }
}
