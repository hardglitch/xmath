use crate::im::core::Im;

impl Im {
    pub fn powi(mut self, mut rhs: Self) -> Self {

        //! Powi only raises the i-value to a power.
        //! # Example
        //!```
        //! use xmath::im::cast::ImValue;
        //!
        //! let expr = 2.i().powi(3.r()); // 2i^3
        //! println!("{}" ,expr);
        //! // -2i
        //!```

        if self.is_none() || rhs.is_none() { return Self::none() }

        self.pow_core(&mut rhs, true);
        self
    }
}