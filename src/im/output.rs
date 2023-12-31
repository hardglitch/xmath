use std::fmt::{Display, Formatter};
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
            div = "1/";
        }

        if self.is_simple_im() && (self.real == 1.0 || self.real == -1.0) {
            real.clear()
        }

        format!("{}{}{}{}", sign, div, real, i)
    }

    fn format_complex(&self) -> String {

        let mut mul = "".to_string();
        let mut sign = "".to_string();
        if let Some(m) = self.mixed_pow() {
            if !m.is_simple() {
                mul = ["(", &m.format_complex(), ")"].concat()
            }
            else if m.is_real() && m.real != 1.0 {
                mul = m.format_simple();
                if m.im_pow >= 0.0 {
                    if self.real >= 0.0 { sign = "+".to_string() }
                    else if self.real < 0.0 { sign = "-".to_string() }
                }
            }
        }

        let mut pow = "".to_string();
        let mut div = "".to_string();
        if let Some(p) = self.mixed_pow() {
            if !p.is_simple() {
                pow = ["(", &p.format_complex(), ")"].concat()
            }
            else if p.is_simple() && p.real > 0.0 && (p.is_real() && p.real != 1.0) {
                pow = ["^", &p.format_simple()].concat();
            }
            else if p.is_simple() && p.real < 0.0 && (p.is_real() && p.real != -1.0) {
                div = "1/".to_string();
                // sign = m.remove(0).to_string();
            }
        }

        let mut base = "".to_string();
        base = ["(", &Self::format_vec(&self.mixed_base), ")"].concat();

        format!("{}{}{}{}{}", sign, div, mul, base, pow)
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
            self.format_complex()
        }
    }
}
