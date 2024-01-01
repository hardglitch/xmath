use crate::im::core::Im;

impl Im {
    pub fn powi(mut self, mut rhs: Self) -> Self {
        if self.is_none() || rhs.is_none() { return Self::none() }

        unsafe { self.pow_core(&mut rhs, true); }
        self
    }
}