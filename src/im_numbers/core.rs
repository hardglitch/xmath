use std::ptr::swap;

#[derive(PartialEq, Debug)]
pub(crate) enum Sign {
    Plus,
    Minus,
    None,
}

#[derive(Debug, Clone, Default)]
pub struct Im {
    pub(crate) real: f64,
    pub(crate) im_pow: f64,
    pub(crate) mixed_base: Option<Vec<Self>>,
    pub(crate) mixed_pow: Option<Vec<Self>>,
    pub(crate) mixed_mul: Option<Vec<Self>>,
}

impl PartialEq for Im {
    fn eq(&self, other: &Self) -> bool {
        self.mixed_pow == other.mixed_pow &&
        self.mixed_base == other.mixed_base &&
        self.mixed_mul == other.mixed_mul &&
        self.real == other.real &&
        self.im_pow == other.im_pow
    }
}

impl Im {

    pub(crate) fn new(real: f64, im_pow: f64) -> Self {
        Self {
            real,
            im_pow,
            mixed_base: Default::default(),
            mixed_pow: Default::default(),
            mixed_mul: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn real_mixed_pow(&self) -> Option<f64> {
        if let Some(v) = &self.mixed_pow &&
            let Some(p) = v.first() && p.im_pow == 0.0
        {
            return Some(p.real)
        }
        None
    }

    pub(crate) fn real_mixed_base(&self) -> Option<f64> {
        if let Some(b) = &self.mixed_base && b.len() == 1 &&
            let Some(e) = b.first() && e.im_pow == 0.0
        {
            return Some(e.real)
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn real_mixed_base_mut(&mut self) -> Option<&mut Self> {
        if let Some(b) = &mut self.mixed_base && b.len() == 1 &&
            let Some(e) = b.first_mut() && e.im_pow == 0.0
        {
            return Some(e)
        }
        None
    }

    pub(crate) fn mixed_base_simple_values(&self) -> Option<(f64, f64)> {
        if let Some(b) = &self.mixed_base && b.len() == 1 &&
            let Some(e) = b.first()
        {
            return Some((e.real, e.im_pow))
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn im_mixed_base(&self) -> Option<f64> {
        if let Some(b) = &self.mixed_base && b.len() == 1 &&
            let Some(e) = b.first() && e.im_pow > 0.0
        {
            return Some(e.im_pow)
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn real_mixed_mul(&self) -> Option<f64> {
        if let Some(v) = &self.mixed_pow &&
            let Some(m) = v.first() && m.im_pow == 0.0
            {
                return Some(m.real)
            }
        None
    }

    pub(crate) fn push_in_mixed_base(&mut self, expr: Self) {
        if let Some(b) = &mut self.mixed_base {
            b.push(expr.clone())
        }
        else {
            self.mixed_base = Some(vec![expr]);
        }
    }

    pub(crate) fn push_in_mixed_pow(&mut self, expr: Self) {
        if let Some(v) = &mut self.mixed_pow &&
            let Some(m) = v.first_mut() {
                m.push_in_mixed_base(expr)
            }
        else {
            self.mixed_pow = Some(vec![expr]);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn set_mixed_pow(&mut self, real: f64, im_pow: f64) {
        self.mixed_pow = Some(vec![Self::new(real, im_pow)])
    }

    pub(crate) fn push_in_mixed_mul(&mut self, expr: Self) {
        if let Some(v) = &mut self.mixed_mul &&
            let Some(m) = v.first_mut() {
                m.push_in_mixed_base(expr)
            }
            else {
                self.mixed_mul = Some(vec![expr]);
            }
    }

    pub(crate) fn set_mixed_mul(&mut self, real: f64, im_pow: f64) {
        self.mixed_mul = Some(vec![Self::new(real, im_pow)])
    }

    pub(crate) fn simple_to_mixed_base(&mut self) {
        if (self.is_real() || self.is_simple_im()) && !self.is_zero() {
            self.push_in_mixed_base(self.clone());
            self.clear_simples();
        }
    }

    pub(crate) fn mixed_base_to_simple(&mut self) {
        if self.is_mixed_base_only() {
            if let Some((r, i)) = self.mixed_base_simple_values() {
                *self = Self::new(r, i);
            }
        }
    }

    pub(crate) fn clear_simples(&mut self) {
        self.real = 0.0;
        self.im_pow = 0.0;
    }

    pub(crate) unsafe fn neg(&mut self) {
        if self.is_simple() {
            self.real = -self.real
        }

        else if self.is_mixed_base_only() {
            if let Some(b) = &mut self.mixed_base {
                b.iter_mut().for_each(|e|
                    e.mul_core(&mut Self::new(-1.0, 0.0))
                )
            }
        }

        else if self.is_mixed_pow_and_base_only() {
            self.set_mixed_mul(-1.0, 0.0)
        }

        else if self.is_mixed_all() &&
            let Some(v) = &mut self.mixed_mul &&
            let Some(m) = v.first_mut()
        {
            m.mul_core(&mut Self::new(-1.0, 0.0))
        }
    }

    pub(crate) fn im_pow_fixer(&mut self) {
        if self.im_pow != 0.0 {
            let pairs = (self.im_pow / 2.0).trunc();
            if pairs != 0.0 && pairs % 2.0 != 0.0 {
                self.real = -self.real
            }
            self.im_pow -= 2.0 * pairs;
        }

        // if let Some(b) = &mut self.mixed_base {
        //     b.iter_mut().for_each(|e| e.pair_checker())
        // }
        // if let Some(p) = &mut self.mixed_pow {
        //     p.iter_mut().for_each(|e| e.pair_checker())
        // }
        // if let Some(m) = &mut self.mixed_mul {
        //     m.iter_mut().for_each(|e| e.pair_checker())
        // }
    }

    pub(crate) fn pow_fixer(&mut self) {
        if let Some(v) = &mut self.mixed_pow &&
            let Some(p) = v.first_mut()
        {
            if p.is_zero() {
                *self = Self::new(1.0, 0.0)
            }
            else if p.is_real() && p.real == 1.0 {
                self.mixed_pow = None
            }
        }
    }

    pub(crate) fn mul_fixer(&mut self) {
        if let Some(v) = &mut self.mixed_pow &&
            let Some(m) = v.first_mut()
        {
            if m.is_zero() { *self = Self::default() }
            else if m.real == 1.0 { self.mixed_mul = None }
        }
    }

    pub(crate) fn simple_fixer(&mut self) {
        if self.is_mixed_base_only() &&
            self.mixed_base_simple_values().is_some_and(|(r, _)| r != 0.0)
        {
            self.mixed_base_to_simple()
        }
    }

    pub(crate) unsafe fn add_ass_mixed_pow(&mut self, rhs: &mut Self) {
        if let Some(v1) = &mut self.mixed_pow &&
            let Some(p1) = v1.first_mut()
        {
            if let Some(v2) = &mut rhs.mixed_pow &&
                let Some(p2) = v2.first_mut()
            {
                p1.add_core(p2);
                self.pow_fixer();
            }
        } else {
            self.push_in_mixed_pow(rhs.clone());
        }
    }

    pub(crate) unsafe fn sub_ass_mixed_pow(&mut self, rhs: &mut Self) {
        if let Some(v1) = &mut self.mixed_pow &&
            let Some(p1) = v1.first_mut()
        {
            if let Some(v2) = &mut rhs.mixed_pow &&
                let Some(p2) = v2.first_mut()
            {
                p1.sub_core(p2);
                self.pow_fixer();
            }
        } else {
            self.push_in_mixed_pow(rhs.clone());
        }
    }

    pub(crate) unsafe fn mul_ass_mixed_pow(&mut self, rhs: &mut Self) {
        if let Some(v1) = &mut self.mixed_pow &&
            let Some(p1) = v1.first_mut()
        {
            if let Some(v2) = &mut rhs.mixed_pow &&
                let Some(p2) = v2.first_mut()
            {
                p1.mul_core(p2);
                self.pow_fixer();
            }
        } else {
            self.push_in_mixed_pow(rhs.clone());
        }
    }

    pub(crate) unsafe fn div_ass_mixed_pow(&mut self, rhs: &mut Self) {
        if let Some(v1) = &mut self.mixed_pow &&
            let Some(p1) = v1.first_mut()
        {
            if let Some(v2) = &mut rhs.mixed_pow &&
                let Some(p2) = v2.first_mut()
            {
                p1.div_core(p2);
                self.pow_fixer();
            }
        } else {
            self.push_in_mixed_pow(rhs.clone());
        }
    }

    pub(crate) unsafe fn mul_ass_mixed_mul(&mut self, rhs: &mut Self) {
        if let Some(v1) = &mut self.mixed_mul &&
            let Some(m1) = v1.first_mut()
        {
            if let Some(v2) = &mut rhs.mixed_mul &&
                let Some(m2) = v2.first_mut()
            {
                m1.mul_core(m2);
                self.mul_fixer();
            }
        } else {
            self.push_in_mixed_mul(rhs.clone());
        }
    }

    pub(crate) unsafe fn div_ass_mixed_mul(&mut self, rhs: &mut Self) {
        if let Some(v1) = &mut self.mixed_mul &&
            let Some(m1) = v1.first_mut()
        {
            if let Some(v2) = &mut rhs.mixed_mul &&
                let Some(m2) = v2.first_mut()
            {
                m1.div_core(m2);
                self.mul_fixer();
            }
        } else {
            self.push_in_mixed_mul(rhs.clone());
        }
    }

    pub(crate) unsafe fn add_ass_mixed_mul(&mut self, rhs: &mut Self) {
        if let Some(v1) = &mut self.mixed_mul &&
            let Some(m1) = v1.first_mut()
        {
            if let Some(v2) = &mut rhs.mixed_mul &&
                let Some(m2) = v2.first_mut()
            {
                m1.add_core(m2);
                self.mul_fixer();
            }
        } else {
            self.push_in_mixed_mul(rhs.clone());
        }
    }

    pub(crate) unsafe fn sub_ass_mixed_mul(&mut self, rhs: &mut Self) {
        if let Some(v1) = &mut self.mixed_mul &&
            let Some(m1) = v1.first_mut()
        {
            if let Some(v2) = &mut rhs.mixed_mul &&
                let Some(m2) = v2.first_mut()
            {
                m1.sub_core(m2);
                self.mul_fixer();
            }
        } else {
            self.push_in_mixed_mul(rhs.clone());
        }
    }

    pub(crate) unsafe fn add_ass_mixed_base(&mut self, rhs: &Self) {
        let mut expr = Self::default();
        expr.push_in_mixed_base(self.clone());
        expr.push_in_mixed_base(rhs.clone());
        *self = expr
    }

    pub(crate) unsafe fn collect(&mut self) {
        if self.is_mixed_base_only() {
            let e = &mut Im::default();

            if let Some(v) = &mut self.mixed_base {
                while !v.is_empty() {
                    if let Some(e1) = v.pop().as_mut() {
                        e.add_core(e1);
                    }
                }
            }
            swap(self, e);
        }
    }
}