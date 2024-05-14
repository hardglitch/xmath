use crate::im::core::Im;

impl Im {
    pub fn pow(mut self, mut rhs: Self) -> Self {

        //! Pow raises an entire expression to a power.
        //! # Example
        //!```
        //! use xmath::im::cast::ImValue;
        //!
        //! let expr = 2.i().pow(3.r()); // (2i)^3
        //! println!("{}" ,expr);
        //! // -8i
        //!```

        if self.is_none() || rhs.is_none() { return Self::none() }

        self.pow_core(&mut rhs, false);
        self
    }

    pub(crate) fn pow_core(&mut self, rhs: &mut Self, is_powi: bool) {
        self.im_pow_fixer();
        rhs.im_pow_fixer();

        self.pow_logic(rhs, is_powi);
        if self.is_none() || rhs.is_none() {
            *self = Self::none();
            return
        }

        self.fixer_pack();
    }

    fn pow_logic(&mut self, rhs: &mut Self, is_powi: bool) {
        if rhs.is_zero() {
            *self = Im::new(1.0, 0.0);
            return
        }
        if (rhs.is_real() && rhs.real == 1.0) ||
           (self.is_real() && self.real == 1.0) ||
            self.is_zero()
        { return }

        if self.is_simple() && rhs.is_real() {
            if (!is_powi && self.is_simple()) || (is_powi && self.is_real()) {
                self.real = self.real.powf(rhs.real);
            }
            if self.is_simple_im() {
                self.im_pow = rhs.real;
                self.im_pow_fixer();
            }
        }

        else if self.is_simple() && rhs.is_simple_im() {
            self.simple_to_mixed_base();
            self.push_in_mixed_pow(rhs.clone());
        }

        else if self.is_mixed_base_only() {
            if rhs.is_real() && rhs.real.fract() == 0.0 {
                let stable = self.clone();
                for _ in 0..(rhs.real.abs() - 1.0) as usize {
                    self.mul_core(&mut stable.clone());
                }
                if rhs.real < 0.0 {
                    self.push_in_mixed_pow(Self::new(-1.0, 0.0));
                }
            }
            else  {
                self.push_in_mixed_pow(rhs.clone())
            }
        }

        else if (self.is_mixed_pow_and_base_only() || self.is_mixed_all()) &&
            let Some(p) = self.mixed_pow_mut()
        {
            p.mul_core(rhs)
        }
    }
}