use std::cmp::{min, Ordering};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::mem::swap;
use std::ops::{Add, Deref, Mul, Sub};
use crate::utils::is_equal;


#[derive(Debug, Clone)]
pub struct Matrix {
    strings: usize,
    rows: usize,
    body: Vec<f64>,
}
impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Matrix ({}x{}) = ({:?})", self.strings, self.rows, self.body)
    }
}
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.strings == other.strings &&
            self.rows == other.rows &&
            self.body
                .iter()
                .enumerate()
                .all(|(i, n)| is_equal(n, &other.body[i], 0.0001))
    }
}
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_by_ref(&rhs).unwrap()
    }
}
impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_by_ref(&rhs).unwrap()
    }
}
impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_by_ref(&rhs).unwrap()
    }
}
impl Matrix {
    pub fn new(strings: usize, rows: usize, body: Vec<f64>) -> Result<Self, Box<dyn Error>> {
        if strings == 0 || rows == 0 { return Err("Arguments must be greater than 0.".into()) }
        if body.len() != strings * rows { return Err("The matrix have an incorrect size.".into()) }

        Ok(Self { strings, rows, body })
    }

    pub fn det(&self) -> f64 {
        //! The matrix determinant.
        //! # Example
        //!```
        //! use xmath::matrices::Matrix;
        //!
        //! let m = Matrix::new(5, 5, vec![
        //!     0.0, 5.0, 6.0, 7.0, 1.0,
        //!     1.0, 4.0, 5.0, 1.0, 1.0,
        //!     0.0, 3.0, 1.0, 2.0, 2.0,
        //!     0.0, 1.0, 7.0, 8.0, 6.0,
        //!     0.0, 1.0, 4.0, 4.0, 7.0,
        //! ]).unwrap();
        //!
        //! println!("det = {:?}", m.det());
        //! // det = 156.0
        //! ```

        if self.rows != self.strings { return 0.0 }

        let mut det = 0_f64;
        if self.body.len() > 4 {
            for s in 0..self.strings {
                let elem = self.body[s * self.rows];
                let ad = (-1_f64).powi((1 + s+1) as i32);
                let new_m = self._sub_matrix(s, 0);
                let minor = new_m.det();
                let d = elem * ad * minor;
                det += d;
            }
        } else {
            let minor = self.body[0]*self.body[3] - self.body[1]*self.body[2];
            det += minor;
        }
        det
    }

    fn _sub_matrix(&self, s: usize, r: usize) -> Self {
        let str = (s * self.rows..s * self.rows + self.rows).collect::<Vec<usize>>();

        let sub_matrix: Vec<f64> = self.body
            .iter()
            .enumerate()
            .filter_map(|(i, e)|
                if (i % self.rows != r) && !str.iter().any(|n| *n == i) {Some(*e)} else {None}
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
        //! use xmath::matrices::Matrix;
        //!
        //! let m1 = Matrix::new(3, 3, vec![
        //!     1.0, 2.0, 3.0,
        //!     4.0, 5.0, 6.0,
        //!     1.0, 2.0, 3.0,
        //! ]).unwrap();
        //!
        //! let m2 = Matrix::new(3, 3, vec![
        //!     1.0, 2.0, 3.0,
        //!     1.0, 2.0, 3.0,
        //!     5.0, 2.0, 1.0,
        //! ]).unwrap();
        //!
        //! println!("m1 * m2 = {:?}", m1.mul_by_ref(&m2).unwrap());
        //! println!("m1 * m2 = {:?}", m1 * m2);
        //! // m1 * m2 = Matrix { strings: 3, rows: 3, body: [18.0, 12.0, 12.0, 39.0, 30.0, 33.0, 18.0, 12.0, 12.0] }
        //! // m1 * m2 = Matrix { strings: 3, rows: 3, body: [18.0, 12.0, 12.0, 39.0, 30.0, 33.0, 18.0, 12.0, 12.0] }
        //! ```

        if self.rows != rhs.strings && rhs.rows != self.strings {
            return Err("Matrices must have the same dimensions.".into())
        }
        let mut left = self;
        let mut right = rhs;
        if left.body.len() < right.body.len() { swap(&mut left, &mut right) }

        let mut new_m= Vec::<f64>::new();
        for s in 0..left.strings {
            let str = (s * left.rows..s * left.rows + left.rows).collect::<Vec<usize>>();

            for r in 0..right.rows {
                let new_elem = str
                    .iter()
                    .enumerate()
                    .map(|(i, n)| left.body[*n] * right.body[r + right.rows * i])
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
        //! use xmath::matrices::Matrix;
        //!
        //! let m = Matrix::new(3, 3, vec![
        //!     1.0, 1.0, 2.0,
        //!     3.0, -4.0, 5.0,
        //!     6.0, 7.0, 8.0,
        //! ]).unwrap();
        //!
        //! let m1 = m.pow(3).unwrap();
        //! let m2 = m.mul_by_ref(&m).unwrap().mul_by_ref(&m).unwrap();
        //! let m3 = m.clone() * m.clone() * m;
        //!
        //! println!("m1 = {:?}", m1);
        //! println!("m2 = {:?}", m2);
        //! println!("m3 = {:?}", m3);
        //! // m1 = Matrix { strings: 3, rows: 3, body: [187.0, 133.0, 271.0, 339.0, -13.0, 520.0, 843.0, 716.0, 1208.0] }
        //! // m2 = Matrix { strings: 3, rows: 3, body: [187.0, 133.0, 271.0, 339.0, -13.0, 520.0, 843.0, 716.0, 1208.0] }
        //! // m3 = Matrix { strings: 3, rows: 3, body: [187.0, 133.0, 271.0, 339.0, -13.0, 520.0, 843.0, 716.0, 1208.0] }
        //! ```

        if self.rows != self.strings {
            return Err("Such matrices must not be raised to a power.".into())
        }
        match pow {
            0 => { Self::new(self.strings, self.rows, vec![0.0; self.body.len()]) }
            1 => { Ok(self.clone()) }
            _ => {
                let mut m = Box::new(self.mul_by_ref(self)?);
                for _ in 2..pow {
                    let old_m = m.deref();
                    let new_m = old_m.mul_by_ref(self)?;
                    *m = new_m;
                }
                Ok(*m)
            }
        }
    }

    pub fn add_by_ref(&self, rhs: &Self) -> Result<Self, Box<dyn Error>> {
        //! Matrix addition by references.
        //! # Example
        //!```
        //! use xmath::matrices::Matrix;
        //!
        //! let m1 = Matrix::new(3, 3, vec![
        //!     1.0, 2.0, 3.0,
        //!     4.0, 5.0, 6.0,
        //!     7.0, 8.0, 9.0,
        //! ]).unwrap();
        //!
        //! let m2 = Matrix::new(3, 3, vec![
        //!     1.0, 1.0, 2.0,
        //!     3.0, -4.0, 5.0,
        //!     6.0, 7.0, 8.0,
        //! ]).unwrap();
        //!
        //! println!("m1 + m2 = {:?}", m1.add_by_ref(&m2).unwrap());
        //! println!("m1 + m2 = {:?}", m1 + m2);
        //! // m1 + m2 = Matrix { strings: 3, rows: 3, body: [2.0, 3.0, 5.0, 7.0, 1.0, 11.0, 13.0, 15.0, 17.0] }
        //! // m1 + m2 = Matrix { strings: 3, rows: 3, body: [2.0, 3.0, 5.0, 7.0, 1.0, 11.0, 13.0, 15.0, 17.0] }
        //! ```

        if self.rows != rhs.rows || self.strings != rhs.strings {
            return Err("Matrices must have the same dimensions.".into())
        }

        let new_m: Vec<f64> = self.body
            .iter()
            .enumerate()
            .map(|(i, e)| e + rhs.body[i])
            .collect();

        Self::new(self.strings, self.rows, new_m)
    }

    pub fn sub_by_ref(&self, rhs: &Self) -> Result<Self, Box<dyn Error>> {
        //! Matrix subtraction by references.
        //! # Example
        //!```
        //! use xmath::matrices::Matrix;
        //!
        //! let m1 = Matrix::new(3, 3, vec![
        //!     1.0, 2.0, 3.0,
        //!     4.0, 5.0, 6.0,
        //!     7.0, 8.0, 9.0,
        //! ]).unwrap();
        //!
        //! let m2 = Matrix::new(3, 3, vec![
        //!     1.0, 1.0, 2.0,
        //!     3.0, -4.0, 5.0,
        //!     6.0, 7.0, 8.0,
        //! ]).unwrap();
        //!
        //! println!("m1 - m2 = {:?}", m1.sub_by_ref(&m2).unwrap());
        //! println!("m1 - m2 = {:?}", m1 - m2);
        //! // m1 - m2 = Matrix { strings: 3, rows: 3, body: [0.0, 1.0, 1.0, 1.0, 9.0, 1.0, 1.0, 1.0, 1.0] }
        //! // m1 - m2 = Matrix { strings: 3, rows: 3, body: [0.0, 1.0, 1.0, 1.0, 9.0, 1.0, 1.0, 1.0, 1.0] }
        //! ```

        if self.rows != rhs.rows || self.strings != rhs.strings {
            return Err("Matrices must have the same dimensions.".into())
        }

        let new_m: Vec<f64> = self.body
            .iter()
            .enumerate()
            .map(|(i, e)| e - rhs.body[i])
            .collect();

        Self::new(self.strings, self.rows, new_m)
    }

    pub fn transpose(&self) -> Self {
        //! Matrix transposition.
        //! # Example
        //!```
        //! use xmath::matrices::Matrix;
        //!
        //! let m = Matrix::new(3, 3, vec![
        //!     1.0, 1.0, 2.0,
        //!     3.0, -4.0, 5.0,
        //!     6.0, 7.0, 8.0,
        //! ]).unwrap();
        //!
        //! println!("Transposed m = {:?}", m.transpose());
        //! // Transposed m = Matrix { strings: 3, rows: 3, body: [1.0, 3.0, 6.0, 1.0, -4.0, 7.0, 2.0, 5.0, 8.0] }
        //! ```

        let mut new_body = self.body.to_vec();
        let mut size = Box::<usize>::new(0);
        match self.rows.cmp(&self.strings) {
            Ordering::Greater => { *size = (self.rows - self.strings) * self.rows }
            Ordering::Less => { *size = (self.strings - self.rows) * self.strings }
            _ => { *size = self.rows }
        }
        for (i, e) in self.body.iter().enumerate(){
            new_body[ i / *size + min(self.rows, self.strings) * (i % *size) ] = *e
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
        //! use xmath::matrices::Matrix;
        //!
        //! let m = Matrix::new(3, 3, vec![
        //!     1.0, 1.0, 2.0,
        //!     3.0, -4.0, 5.0,
        //!     6.0, 7.0, 8.0,
        //! ]).unwrap();
        //! let im = m.inverse().unwrap();
        //!
        //! println!("m * im = im * m: {:?}", m.mul_by_ref(&im).unwrap() == im.mul_by_ref(&m).unwrap());
        //! // or
        //! // println!("m * im = im * m: {:?}", m.clone() * im.clone() == im.clone() * m.clone());
        //! // m * im = im * m: true
        //!
        //! println!("m * im = {:?}", m * im);
        //! // m * im = Matrix { strings: 3, rows: 3, body: [1.0, 0.0, 5.551115123125783e-17, 0.0, 1.0, 0.0, 0.0, 1.1102230246251565e-16, 1.0] }
        //!
        //! // 1
        //! //  1    - identity matrix
        //! //   1
        //! ```

        if self.rows != self.strings { return None }

        let det = self.det();
        if det == 0.0 { return None }

        let mut ads = Vec::<f64>::new();

        self._cofactor_matrix(&mut ads);
        let size = ads.len().isqrt();

        Some(
            Self::new(size,size,ads
                .to_vec()).unwrap()
                .transpose()
                .mul_num(1.0 / det)
        )
    }

    pub fn mul_num(&self, num: f64) -> Self {
        //! Matrix multiplication by number.
        //! # Example
        //!```
        //! use xmath::matrices::Matrix;
        //!
        //! let m = Matrix::new(3, 3, vec![
        //!     1.0, 2.0, 3.0,
        //!     1.0, 2.0, 3.0,
        //!     5.0, 2.0, 1.0,
        //! ]).unwrap();
        //!
        //! println!("m * 2 = {:?}", m.mul_num(2.0));
        //! // m * 2 = Matrix { strings: 3, rows: 3, body: [2.0, 4.0, 6.0, 2.0, 4.0, 6.0, 10.0, 4.0, 2.0] }
        //! ```

        let new_m: Vec<f64> = self.body
            .iter()
            .map(|e| e * num)
            .collect();

        Self::new(self.strings, self.rows, new_m).unwrap()
    }

    pub fn cofactor_matrix(&self) -> Option<Self> {
        //! Cofactor matrix.
        //! # Example
        //!```
        //! use xmath::matrices::Matrix;
        //!
        //! let m = Matrix::new(3, 3, vec![
        //!     1.0, 1.0, 2.0,
        //!     3.0, -4.0, 5.0,
        //!     6.0, 7.0, 8.0,
        //! ]).unwrap();
        //!
        //! println!("Cofactor matrix of m = {:?}", m.cofactor_matrix().unwrap());
        //! // Cofactor matrix of m = Matrix { strings: 3, rows: 3, body: [-67.0, 6.0, 45.0, 6.0, -4.0, -1.0, 13.0, 1.0, -7.0] }
        //! ```

        if self.rows != self.strings { return None }

        let mut ads = Vec::<f64>::new();
        self._cofactor_matrix(&mut ads);
        let size = ads.len().isqrt();
        Some(Self::new(size, size, ads).unwrap())
    }

    fn _cofactor_matrix(&self, ads: &mut Vec<f64>) {
        if self.body.len() >= 4 {
            for s in 0..self.strings {
                for r in 0..self.rows {
                    let ad = (-1_f64).powi((r+1 + s+1) as i32);
                    let new_m = self._sub_matrix(s, r);
                    let minor = new_m.det();
                    let d = ad * minor;
                    ads.push(d);
                }
            }
        }
    }

    pub fn slae(&self, d: &[f64]) -> Result<Option<Vec<f64>>, Box<dyn Error>> {
        //! SLAE (System of Linear (Algebraic) Equations) (Kramer's method).
        //! # Example
        //! ```
        //! use xmath::matrices::Matrix;
        //!
        //! let m = Matrix::new(3, 3, vec![
        //!     1.0, 4.0, 2.0,
        //!     2.0, -6.0, -2.0,
        //!     1.0, 5.0, 2.0,
        //! ]).unwrap();
        //!
        //! let d = &[1.0, 3.0, 2.0];
        //!
        //! for (i, x) in m.slae(d).unwrap().unwrap().iter().enumerate() {
        //!     println!("x{} = {:?}", i+1, x);
        //! }
        //! // x1 = 2.0
        //! // x2 = 1.0
        //! // x3 = -2.5
        //! ```

        if self.rows != self.strings { return Ok(None) }
        if d.len() != self.strings { return Err("The number of d-elements is not equal to the number of strings in the Matrix.".into()) }

        let det = self.det();
        if det == 0.0 { return Ok(None) }

        let mut res = Vec::<f64>::new();

        for r in 0..self.rows {
            let mut new_body = self.body.to_vec();
            for (i, e) in d.iter().enumerate() {
                new_body[r + self.rows * i] = *e
            }
            let size = new_body.len().isqrt();
            let x_m = Self::new(size,size, new_body)?;
            let x = x_m.det() / det;
            res.push(x);
        }

        Ok(Some(res))
    }
}
