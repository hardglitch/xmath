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

    pub(crate) fn pair_checker(&mut self) {
        if self.im_pow != 0.0 {
            let pairs = (self.im_pow / 2.0).trunc();
            if pairs != 0.0 && pairs % 2.0 != 0.0 {
                self.real = -self.real
            }
            self.im_pow -= 2.0 * pairs;
        }

        if let Some(b) = &mut self.mixed_base {
            b.iter_mut().for_each(|e| e.pair_checker())
        }
        if let Some(p) = &mut self.mixed_pow {
            p.iter_mut().for_each(|e| e.pair_checker())
        }
        if let Some(m) = &mut self.mixed_mul {
            m.iter_mut().for_each(|e| e.pair_checker())
        }
    }

    pub(crate) fn is_real(&self) -> bool {
        self.mixed_base.is_none() &&
        self.mixed_pow.is_none() &&
        self.mixed_mul.is_none() &&
        self.im_pow == 0.0
    }

    pub(crate) fn is_simple_im(&self) -> bool {
        self.mixed_base.is_none() &&
        self.mixed_pow.is_none() &&
        self.mixed_mul.is_none() &&
        self.im_pow > 0.0
    }

    pub(crate) fn is_mixed_im(&self) -> bool {
        self.mixed_base.is_some() && self.real == 0.0 && self.im_pow == 0.0
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.mixed_base.is_none() &&
        self.mixed_pow.is_none() &&
        self.mixed_mul.is_none() &&
        self.real == 0.0
    }

    #[allow(unused_variables)]
    pub(crate) fn is_mixed_base_simple(&self) -> bool {
        self.is_mixed_base_only() &&
        self.mixed_base_simple_values().is_some_and(|(r, i)| r != 0.0)
    }

    #[allow(dead_code)]
    pub(crate) fn is_equal_by_abs(&self, other: &Self) -> Sign {
        let mut neg_self = self.clone();
        neg_self.neg();
        if self == other { Sign::Plus }
        else if &neg_self == other { Sign::Minus }
        else { Sign::None }
    }

    pub(crate) fn is_mixed_mul_variable(&self, other: &Self) -> bool {
        self.mixed_base == other.mixed_base && self.mixed_base.is_some() &&
        self.mixed_pow == other.mixed_pow && self.mixed_pow.is_some()
    }

    pub(crate) fn is_mixed_pow_variable(&self, other: &Self) -> bool {
        self.mixed_base == other.mixed_base && self.mixed_base.is_some() &&
        self.mixed_mul == other.mixed_mul && self.mixed_pow.is_none()
    }

    #[allow(dead_code)]
    pub(crate) fn is_mixed_pow_len_big(&self) -> bool {
        self.mixed_pow.as_ref().is_some_and(|e| e.len() > 1)
    }

    #[allow(dead_code)]
    pub(crate) fn is_mixed_mul_len_big(&self) -> bool {
        self.mixed_mul.as_ref().is_some_and(|e| e.len() > 1)
    }

    pub(crate) fn is_mixed_base_len_big(&self) -> bool {
        self.mixed_base.as_ref().is_some_and(|e| e.len() > 1)
    }

    #[allow(dead_code)]
    pub(crate) fn is_mixed_base_len_bigger(&self, other: &Self) -> bool {
        if let Some(b1) = &self.mixed_base &&
            let Some(b2) = &other.mixed_base &&
            b1.len() > b2.len()
        { return true }
        false
    }

    pub(crate) fn is_mixed_base_only(&self) -> bool {
        self.mixed_mul.is_none() && self.mixed_pow.is_none() && self.mixed_base.is_some()
    }

    pub(crate) fn is_mixed_pow_and_base_only(&self) -> bool {
        self.mixed_mul.is_none() && self.mixed_pow.is_some() && self.mixed_base.is_some()
    }

    pub(crate) fn is_mixed_all(&self) -> bool {
        self.mixed_mul.is_some() && self.mixed_pow.is_some() && self.mixed_base.is_some()
    }

    pub(crate) fn is_simple_logic(&self, rhs: &Self) -> bool {
        self.mixed_mul.is_none() && self.mixed_pow.is_none() && self.mixed_base.is_none() &&
        rhs.mixed_mul.is_none() && rhs.mixed_pow.is_none() && rhs.mixed_base.is_none()
    }

    pub(crate) fn is_mixed_base_logic(&self, rhs: &Self) -> bool {
        (self.mixed_base.is_some() || rhs.mixed_base.is_some()) &&
        self.mixed_pow.is_none() && rhs.mixed_pow.is_none() &&
        self.mixed_mul.is_none() && rhs.mixed_mul.is_none()
    }

    pub(crate) fn is_mixed_pow_logic(&self, rhs: &Self) -> bool {
        self.mixed_base.is_some() && rhs.mixed_base.is_some() &&
        (self.mixed_pow.is_some() || rhs.mixed_pow.is_some()) &&
        self.mixed_mul.is_none() && rhs.mixed_mul.is_none()
    }

    pub(crate) fn is_mixed_mul_logic(&self, rhs: &Self) -> bool {
        self.mixed_base.is_some() && rhs.mixed_base.is_some() &&
        self.mixed_pow.is_some() && rhs.mixed_pow.is_some() &&
        (self.mixed_mul.is_none() || rhs.mixed_mul.is_none())
    }

    pub(crate) fn is_vec_greater(lhs: &Option<Vec<Im>>, rhs: &Option<Vec<Im>>) -> bool {
        if let Some(v1) = lhs &&
           let Some(v2) = rhs && v1.len() > v2.len()
        { return true }
        false
    }
    #[allow(dead_code)]
    pub(crate) fn real_mixed_pow(&self) -> Option<f64> {
        if let Some(p) = &self.mixed_pow && p.len() == 1 &&
           let Some(e) = p.first() && e.im_pow == 0.0
        {
            return Some(e.real)
        }
        None
    }

    pub(crate) fn real_mixed_pow_mut(&mut self) -> Option<&mut Self> {
        if let Some(p) = &mut self.mixed_pow && p.len() == 1 &&
            let Some(e) = p.first_mut() && e.im_pow == 0.0
        {
            return Some(e)
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

    #[allow(unused_variables)]
    pub(crate) fn mixed_base_simple_values(&self) -> Option<(f64, f64)> {
        if let Some(b) = &self.mixed_base && b.len() == 1 &&
            let Some(e) = b.first()
        {
            return Some((e.real, e.im_pow))
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

    pub(crate) fn im_pow_mixed_base(&self) -> Option<f64> {
        if let Some(b) = &self.mixed_base && b.len() == 1 &&
            let Some(e) = b.first()
            // let Some(b) = &e.mixed_base && b.len() == 1 &&
            // let Some(e) = b.first()
        {
            return Some(e.im_pow)
        }
        None
    }

    pub(crate) fn real_mixed_mul(&self) -> Option<f64> {
        if let Some(m) = &self.mixed_mul &&
            let Some(e) = m.first() && e.im_pow == 0.0
        {
            return Some(e.real)
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn real_mixed_mul_mut(&mut self) -> Option<&mut Self> {
        if let Some(m) = &mut self.mixed_mul && m.len() == 1 &&
            let Some(e) = m.first_mut() && e.im_pow == 0.0
        {
            return Some(e)
        }
        None
    }

    pub(crate) fn push_in_mixed_base(&mut self, expr: Self) {
        if self.mixed_base.is_none() {
            self.mixed_base = Some(vec![expr]);
        }

        else if let Some(b) = &mut self.mixed_base {
            b.push(expr.clone())
        }
    }

    #[allow(dead_code)]
    pub(crate) fn push_in_mixed_pow(&mut self, expr: Self) {
        if self.mixed_pow.is_none() {
            self.mixed_pow = Some(vec![expr]);
        }

        else if let Some(b) = &mut self.mixed_pow {
            b.push(expr)
        }
    }

    pub(crate) fn push_in_mixed_mul(&mut self, expr: Self) {
        if self.mixed_mul.is_none() {
            self.mixed_mul = Some(vec![expr]);
        }

        else if let Some(b) = &mut self.mixed_mul {
            b.push(expr)
        }
    }

    pub(crate) fn clear_mixed_base(&mut self) {
        if self.mixed_base.is_some() { self.mixed_base = None }
    }

    pub(crate) fn clear_simples(&mut self) {
        self.real = 0.0;
        self.im_pow = 0.0;
    }

    pub(crate) fn neg(&mut self) {
        if self.is_real() || self.is_simple_im() {
            self.real = -self.real
        }

        else if self.is_mixed_base_only() {
            if let Some(b) = &mut self.mixed_base {
                b.iter_mut().for_each(|e| e.neg())
            }
        }

        else if self.is_mixed_pow_and_base_only() {
            self.mixed_mul = Some(vec![Self::new(-1.0, 0.0)])
        }

        else if self.is_mixed_all() {
            if let Some(m) = &mut self.mixed_base {
                m.iter_mut().for_each(|e| e.neg())
            }
        }
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

}
