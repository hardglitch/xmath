use std::fmt::{Display, Formatter};
use std::mem::swap;
use crate::im::core::Im;

impl Display for Im {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Im {
    fn format_simple(&self) -> String {
        let mut real = self.real.to_string();

        let mut i = "";
        if self.is_simple_im() { i = "i" }

        let mut sign = "".to_string();
        if self.real >= 0.0 {
            sign = "+".to_string()
        }
        else if self.real < 0.0 {
            sign = real.remove(0).to_string()
        }

        let mut div = "";
        if self.is_simple_im() && self.im_pow < 0.0 {
            div = "/";
        }

        if self.is_simple_im() && (self.real == 1.0 || self.real == -1.0) {
            real.clear()
        }

        format!("{}{}{}{}", sign, real, div, i)
    }

    fn format_complex(&self) -> String {

        let mut mul = "".to_string();
        let mut sign = "".to_string();
        if let Some(m) = self.mixed_mul() {
            if !m.is_simple() {
                mul = m.format_complex();
                if mul.find('+') == Some(0) {
                    mul.remove(0);
                }
            }
            else if m.is_real() && m.real != 1.0 {
                mul = m.format_simple();
                if m.im_pow >= 0.0 {
                    if self.real >= 0.0 { sign = "+".to_string() }
                    else if self.real < 0.0 && &mul[..1] != "-" { sign = "-".to_string() }
                }
            }
        }

        let mut pow = "".to_string();
        let mut div = "".to_string();
        if let Some(p) = self.mixed_pow() {
            if !p.is_simple() {
                pow = p.format_complex();
                if pow.find('+') == Some(0) {
                    pow.remove(0);
                }
                if (p.is_simple() && p.real < 0.0) || p.is_mul_neg() {
                    div = "/".to_string();
                    swap(&mut mul, &mut div);
                }
            }
            else if p.is_simple() && p.real > 0.0 && !(p.is_real() && p.real == 1.0) {
                pow = p.format_simple();
                if pow.find('+') == Some(0) {
                    pow.remove(0);
                }
                pow = ["^", &pow].concat();
            }
            else if p.is_simple() && p.real < 0.0 {
                if p.is_real() && p.real != -1.0 && self.mixed_mul.is_none() {
                    div = "1/".to_string();
                } else {
                    div = "/".to_string();
                    swap(&mut mul, &mut div);
                }
            }
        }

        let mut base = Self::format_vec(&self.mixed_base);
        if base.find('+') == Some(0) {
            base.remove(0);
        }
        base = ["(", &base, ")"].concat();

        format!("{}{}{}{}{}", sign, div, mul, base, pow)
    }

    fn is_mul_neg(&self) -> bool {
        if let Some(m) = self.mixed_mul() && m.is_simple() && m.real < 0.0 {
            return true
        }
        self.is_mul_neg();
        false
    }

    fn format_vec(vec: &Option<Vec<Im>>) -> String {
        if let Some(v) = vec {
            return v.iter().map(|e| e.format_simple()).collect()
        }
        "".to_string()
    }

    pub(crate) fn format(&self) -> String {
        if self.is_none() { return "None".to_string() }

        if self.is_simple() {
            let mut s = self.format_simple();
            if self.real >= 0.0 { s.remove(0); }
            s
        }
        else {
            let mut s = self.format_complex();
            if &s[..1] == "+" {
                s.remove(0);
            }
            s
        }
    }
}
