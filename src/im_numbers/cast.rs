use crate::im_numbers::im_output::Im;

pub trait ImValue<T> {
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
