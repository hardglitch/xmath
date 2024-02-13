use std::cmp::{min, Ordering};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::mem::swap;
use std::ops::{Add, Mul, Sub};
use crate::im::core::Im;


#[derive(Debug, Clone, Default)]
pub struct ImMatrix {
    strings: usize,
    rows: usize,
    body: Vec<Im>,
}

impl Display for ImMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}
impl PartialEq for ImMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.strings == other.strings &&
            self.rows == other.rows &&
            self.body == other.body
    }
}
impl Mul for ImMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_by_ref(&rhs).unwrap()
    }
}
impl Add for ImMatrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_by_ref(&rhs).unwrap()
    }
}
impl Sub for ImMatrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_by_ref(&rhs).unwrap()
    }
}
impl ImMatrix {
    pub fn new(strings: usize, rows: usize, body: Vec<Im>) -> Result<Self, Box<dyn Error>> {
        if strings == 0 || rows == 0 { return Err("Arguments must be greater than 0.".into()) }
        if body.len() != strings * rows { return Err("The matrix have an incorrect size.".into()) }

        Ok(Self { strings, rows, body })
    }

    pub fn det(&self) -> Im {
        //! The matrix determinant.
        //! # Example
        //!```
        //! use xmath::im::im_matrices::ImMatrix;
        //! use xmath::im::cast::ImValue;
        //!
        //! let m = ImMatrix::new(2, 2, vec![
        //!    (-1).r() - 3.i(), 4.r() + 2.i(),
        //!    1.r() + 1.i(), (-2).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! println!("det = {}", m.det());
        //! // det = (3-i)
        //! ```

        if self.rows != self.strings { return Im::new(0.0, 0.0) }

        let mut det = Im::default();
        if self.body.len() > 4 {
            for s in 0..self.strings {
                let elem = self.body[s * self.rows].clone();
                let ad = Im::new((-1_f64).powi((1 + s+1) as i32), 0.0);
                let new_m = self._sub_matrix(s, 0);
                let minor = new_m.det();
                let d = elem * ad * minor;
                det = det + d;
            }
        } else {
            let minor = self.body[0].clone() * self.body[3].clone() - self.body[1].clone() * self.body[2].clone();
            det = det + minor;
        }
        det
    }

    fn _sub_matrix(&self, s: usize, r: usize) -> Self {
        let str = (s * self.rows..s * self.rows + self.rows).collect::<Vec<usize>>();

        let sub_matrix: Vec<Im> = self.body
            .iter()
            .enumerate()
            .filter_map(|(i, e)|
                if (i % self.rows != r) && !str.iter().any(|n| *n == i) {Some(e.clone())} else {None}
            )
            .collect();

        let size = sub_matrix.len().isqrt();

        Self::new(
            size,
            size,
            sub_matrix
        ).unwrap()
    }

    pub fn mul_by_ref(&self, rhs: &Self) -> Result<Self, Box<dyn Error>> {
        //! Matrix multiplication by references.
        //! # Example
        //!```
        //! use xmath::im::im_matrices::ImMatrix;
        //! use xmath::im::cast::ImValue;
        //!
        //! let m1 = ImMatrix::new(2, 2, vec![
        //!    (-1).r() - 3.i(), 4.r() + 2.i(),
        //!    1.r() + 1.i(), (-2).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! let m2 = ImMatrix::new(2, 2, vec![
        //!    (-5).r() - 3.i(), 4.r() + 2.i(),
        //!    4.r() + 3.i(), (-8).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! println!("m1 * m2 = {}", m1.mul_by_ref(&m2).unwrap());
        //! println!("m1 * m2 = {}", m1 * m2);
        //! // m1 * m2 = Matrix (2x2) = [
        //! //  (6+38i) (-32-26i)
        //! //  (-32-26i) (-13-10i)
        //! // ]
        //! // m1 * m2 = Matrix (2x2) = [
        //! //  (6+38i) (-32-26i)
        //! //  (-32-26i) (-13-10i)
        //! // ]
        //! ```

        if self.rows != rhs.strings && rhs.rows != self.strings {
            return Err("Matrices must have the same dimensions.".into())
        }
        let mut left = self;
        let mut right = rhs;
        if left.body.len() < right.body.len() { swap(&mut left, &mut right) }

        let mut new_m= Vec::<Im>::new();
        for s in 0..left.strings {
            let str = (s * left.rows..s * left.rows + left.rows).collect::<Vec<usize>>();

            for r in 0..right.rows {
                let new_elem = str
                    .iter()
                    .enumerate()
                    .map(|(i, n)| left.body[*n].clone() * right.body[r + right.rows * i].clone())
                    .sum();

                new_m.push(new_elem);
            }
        }
        Self::new(left.strings, right.rows, new_m)
    }

    pub fn pow(&self, pow: usize) -> Result<Self, Box<dyn Error>> {
        //! Matrix exponentiation.
        //! # Example
        //! ```
        //! use xmath::im::cast::ImValue;
        //! use xmath::im::im_matrices::ImMatrix;
        //!
        //! let m = ImMatrix::new(2, 2, vec![
        //!     (-5).r() - 3.i(), 4.r() + 2.i(),
        //!     4.r() + 3.i(), (-8).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! println!("m^3 = {}", m.pow(3).unwrap());
        //! // m^3 = Matrix (2x2) = [
        //! //  (-608i-70) (422+476i)
        //! //  (422+476i) (369+608i)
        //! // ]
        //! ```

        if self.rows != self.strings {
            return Err("Such matrices must not be raised to a power.".into())
        }
        match pow {
            0 => { Self::new(self.strings, self.rows, vec![Im::new(0.0, 0.0); self.body.len()]) }
            1 => { Ok(self.clone()) }
            _ => {
                let mut m = Box::new(self.mul_by_ref(self)?);
                let mut p = 2_usize;

                loop {
                    let mut new_m = Box::<ImMatrix>::default();
                    match (pow - p).cmp(&1) {
                        Ordering::Equal => {
                            *new_m = m.mul_by_ref(self)?;
                            *m = *new_m;
                            break;
                        },
                        Ordering::Greater => {
                            *new_m = m.mul_by_ref(&m)?;
                            *m = *new_m;
                            p += p;
                        },
                        _ => break,
                    }
                }
                Ok(*m)
            }
        }
    }

    pub fn add_by_ref(&self, rhs: &Self) -> Result<Self, Box<dyn Error>> {
        //! Matrix addition by references.
        //! # Example
        //!```
        //! use xmath::im::cast::ImValue;
        //! use xmath::im::im_matrices::ImMatrix;
        //!
        //! let m1 = ImMatrix::new(2, 2, vec![
        //!     (-1).r() - 3.i(), 4.r() + 2.i(),
        //!     1.r() + 1.i(), (-2).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! let m2 = ImMatrix::new(2, 2, vec![
        //!     (-5).r() - 3.i(), 4.r() + 2.i(),
        //!     4.r() + 3.i(), (-8).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! println!("m1 + m2 = {}", m1.add_by_ref(&m2).unwrap());
        //! println!("m1 + m2 = {}", m1 + m2);
        //! // m1 + m2 = Matrix (2x2) = [
        //! //  (-6i-6) (4i+8)
        //! //  (4i+8) (4i+5)
        //! // ]
        //! // m1 + m2 = Matrix (2x2) = [
        //! //  (-6i-6) (4i+8)
        //! //  (4i+8) (4i+5)
        //! // ]
        //! ```

        if self.rows != rhs.rows || self.strings != rhs.strings {
            return Err("Matrices must have the same dimensions.".into())
        }

        let new_m: Vec<Im> = self.body
            .iter()
            .enumerate()
            .map(|(i, e)| e.clone() + rhs.body[i].clone())
            .collect();

        Self::new(self.strings, self.rows, new_m)
    }

    pub fn sub_by_ref(&self, rhs: &Self) -> Result<Self, Box<dyn Error>> {
        //! Matrix subtraction by references.
        //! # Example
        //!```
        //! use xmath::im::cast::ImValue;
        //! use xmath::im::im_matrices::ImMatrix;
        //!
        //! let m1 = ImMatrix::new(2, 2, vec![
        //!     (-1).r() - 3.i(), 4.r() + 2.i(),
        //!     1.r() + 1.i(), (-2).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! let m2 = ImMatrix::new(2, 2, vec![
        //!     (-5).r() - 3.i(), 4.r() + 2.i(),
        //!     4.r() + 3.i(), (-8).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! println!("m1 - m2 = {}", m1.sub_by_ref(&m2).unwrap());
        //! println!("m1 - m2 = {}", m1 - m2);
        //! // m1 - m2 = Matrix (2x2) = [
        //! // 4 0
        //! // 0 (-2i-3)
        //! // ]
        //! // m1 - m2 = Matrix (2x2) = [
        //! // 4 0
        //! // 0 (-2i-3)
        //! // ]
        //! ```

        if self.rows != rhs.rows || self.strings != rhs.strings {
            return Err("Matrices must have the same dimensions.".into())
        }

        let new_m: Vec<Im> = self.body
            .iter()
            .enumerate()
            .map(|(i, e)| e.clone() - rhs.body[i].clone())
            .collect();

        Self::new(self.strings, self.rows, new_m)
    }

    pub fn transpose(&self) -> Self {
        //! Matrix transposition.
        //! # Example
        //!```
        //! use xmath::im::im_matrices::ImMatrix;
        //! use xmath::im::cast::ImValue;
        //!
        //! let m = ImMatrix::new(2, 2, vec![
        //!    (-1).r() - 3.i(), 4.r() + 2.i(),
        //!    1.r() + 1.i(), (-2).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! println!("Transposed m = {}", m.transpose());
        //! // Transposed m = Matrix (2x2) = [
        //! //  (-1-3i) (1+i)
        //! //  (1+i) (4+2i)
        //! // ]
        //! ```

        let mut new_body = self.body.to_vec();
        let mut size = Box::<usize>::new(0);
        match self.rows.cmp(&self.strings) {
            Ordering::Greater => { *size = (self.rows - self.strings) * self.rows }
            Ordering::Less => { *size = (self.strings - self.rows) * self.strings }
            _ => { *size = self.rows }
        }
        for (i, e) in self.body.iter().enumerate(){
            new_body[ i / *size + min(self.rows, self.strings) * (i % *size) ] = e.clone()
        }
        Self::new(self.rows, self.strings, new_body).unwrap()
    }

    pub fn inverse(&self) -> Option<Self> {
        //! Matrix inversion.
        //! # Example
        //! ```
        //!// inv_m = transposed CFM / det,
        //!//     CFM - cofactor matrix
        //!//     det - matrix determinant
        //!
        //! use xmath::im::im_matrices::ImMatrix;
        //! use xmath::im::cast::ImValue;
        //!
        //! let m = ImMatrix::new(2, 2, vec![
        //!    (-1).r() - 3.i(), 4.r() + 2.i(),
        //!    1.r() + 1.i(), (-2).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! println!("im = {}", m.inverse().unwrap());
        //! // im = Matrix (2x2) = [
        //! //  (i-2)/(3-i) (-2i-4)/(3-i)
        //! //  (-2i-4)/(3-i) (i+1)/(3-i)
        //! // ]
        //! ```

        if self.rows != self.strings { return None }

        let det = self.det();
        if det.is_zero() { return None }

        let mut ads = Vec::<Im>::new();

        self._cofactor_matrix(&mut ads);
        let size = ads.len().isqrt();

        Some(
            Self::new(size,size,ads
                .to_vec()).unwrap()
                .transpose()
                .mul_num(Im::new(1.0, 0.0) / det)
        )
    }

    pub fn mul_num(&self, num: Im) -> Self {
        //! Matrix multiplication by number.
        //! # Example
        //!```
        //! use xmath::im::im_matrices::ImMatrix;
        //! use xmath::im::cast::ImValue;
        //!
        //! let m = ImMatrix::new(2, 2, vec![
        //!    (-1).r() - 3.i(), 4.r() + 2.i(),
        //!    1.r() + 1.i(), (-2).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! println!("m * 2 = {}", m.mul_num(2.r()));
        //! // m * 2 = Matrix (2x2) = [
        //! //  (-6i-2) (4i+8)
        //! //  (4i+8) (2i+2)
        //! // ]
        //! ```

        let new_m: Vec<Im> = self.body
            .iter()
            .map(|e| e.clone() * num.clone())
            .collect();

        Self::new(self.strings, self.rows, new_m).unwrap()
    }

    pub fn cofactor_matrix(&self) -> Option<Self> {
        //! Cofactor matrix.
        //! # Example
        //!```
        //! use xmath::im::im_matrices::ImMatrix;
        //! use xmath::im::cast::ImValue;
        //!
        //! let m = ImMatrix::new(2, 2, vec![
        //!    (-1).r() - 3.i(), 4.r() + 2.i(),
        //!    1.r() + 1.i(), (-2).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! println!("Cofactor matrix of m = {}", m.cofactor_matrix().unwrap());
        //! // Cofactor matrix of m = Matrix (2x2) = [
        //! //  (i-2) (i+1)
        //! //  (i+1) (-2i-4)
        //! // ]
        //! ```

        if self.rows != self.strings { return None }

        let mut ads = Vec::<Im>::new();
        self._cofactor_matrix(&mut ads);
        let size = ads.len().isqrt();
        Some(Self::new(size, size, ads).unwrap())
    }

    #[allow(unused_assignments)]
    fn _cofactor_matrix(&self, ads: &mut Vec<Im>) {
        let mut minor = Im::default();
        if self.body.len() >= 4 {
            for s in 0..self.strings {
                for r in 0..self.rows {
                    let ad = Im::new((-1_f64).powi((1 + s+1) as i32), 0.0);
                    if self.body.len() > 4 {
                        let new_m = self._sub_matrix(s, r);
                        minor = new_m.det();
                    } else {
                        minor = self.body[self.body.len()-1 - s * self.strings - r].clone();
                    }
                    let d = ad * minor;
                    ads.push(d);
                }
            }
        }
    }

    pub fn slae(&self, d: &[Im]) -> Result<Option<Vec<Im>>, Box<dyn Error>> {
        //! SLAE (System of Linear (Algebraic) Equations) (Kramer's method).
        //! # Example
        //! ```
        //! use xmath::im::im_matrices::ImMatrix;
        //! use xmath::im::cast::ImValue;
        //!
        //! let m = ImMatrix::new(2, 2, vec![
        //!    (-1).r() - 3.i(), 4.r() + 2.i(),
        //!    1.r() + 1.i(), (-2).r() + 1.i(),
        //! ]).unwrap();
        //!
        //! let d = &[1.r() - 1.i(), 2.r() + 1.i()];
        //!
        //! for (i, x) in m.slae(d).unwrap().unwrap().iter().enumerate() {
        //!     println!("x{} = {}", i+1, x);
        //! }
        //! // x1 = (-7-5i)/(3-i)
        //! // x2 = (-1-7i)/(3-i)
        //! ```

        if self.rows != self.strings { return Ok(None) }
        if d.len() != self.strings { return Err("The number of d-elements is not equal to the number of strings in the Matrix.".into()) }

        let det = self.det();
        if det.is_zero() { return Ok(None) }

        let mut res = Vec::<Im>::new();

        for r in 0..self.rows {
            let mut new_body = self.body.to_vec();
            for (i, e) in d.iter().enumerate() {
                new_body[r + self.rows * i] = e.clone()
            }
            let size = new_body.len().isqrt();
            let x_m = Self::new(size,size, new_body)?;
            let x = x_m.det() / det.clone();
            res.push(x);
        }

        Ok(Some(res))
    }

    fn format(&self) -> String {
        let mut str = "\n".to_string();
        for s in 0..self.strings {
            for r in 0..self.rows {
                str = [str, self.body[s+r].to_string()].join(" ");
            }
            str = [str, "\n".to_string()].concat();
        }

        format!("Matrix ({}x{}) = [{}]", self.strings, self.rows, str)
    }
}
