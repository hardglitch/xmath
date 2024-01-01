use crate::im::core::Im;

pub trait ImValue<T> {

    //! Import this trait to implement im-power!
    //!
    //! .i() - imaginary number
    //! .r() - real number
    //!
    //! Use standard operators such as +, -, *, /.
    //!
    //! # Example
    //!```
    //! use xmath::im::cast::ImValue;
    //!
    //! println!("{}" , 2.i() + 3.r());
    //! // (2i+3)
    //!```

    fn r(self) -> T;
    fn i(self) -> T;
}

impl ImValue<Im> for f64 {
    fn r(self) -> Im {
        if self != 0.0 { Im::new(self, 0.0) } else { Im::new(0.0, 0.0) }
    }

    fn i(self) -> Im {
        if self != 0.0 { Im::new(self, 1.0) } else { Im::new(0.0, 0.0) }
    }
}

impl ImValue<Im> for i32 {
    fn r(self) -> Im {
        if self != 0 { Im::new(self as f64, 0.0) } else { Im::new(0.0, 0.0) }
    }

    fn i(self) -> Im {
        if self != 0 { Im::new(self as f64, 1.0) } else { Im::new(0.0, 0.0) }
    }
}
