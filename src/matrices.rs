use std::error::Error;
use std::fmt::{Display, Formatter};
use std::mem::swap;
use std::ops::{Add, Mul, Sub};
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
        self.mul_by_ref(&rhs)
    }
}
impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_by_ref(&rhs)
    }
}
impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_by_ref(&rhs)
    }
}

impl Matrix {
    pub fn new(strings: usize, rows: usize, body: Vec<f64>) -> Result<Self, Box<dyn Error>> {
        if strings == 0 || rows == 0 { return Err("Arguments must be greater than 0.".into()) }
        if body.len() != strings * rows { return Err("The matrix have an incorrect size.".into()) }

        Ok(Self { strings, rows, body })
    }

    pub fn det(&self) -> f64 {
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

    pub fn mul_by_ref(&self, rhs: &Self) -> Self {
        if self.rows != rhs.strings && rhs.rows != self.strings {
            panic!("Matrices must have the same dimensions.");
        }
        let mut left = &self;
        let mut right = &rhs;
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
        Self::new(left.strings, right.rows, new_m).unwrap()
    }

    pub fn add_by_ref(&self, rhs: &Self) -> Self {
        if self.rows != rhs.rows || self.strings != rhs.strings {
            panic!("Matrices must have the same dimensions.")
        }

        let new_m: Vec<f64> = self.body
            .iter()
            .enumerate()
            .map(|(i, e)| e + rhs.body[i])
            .collect();

        Self::new(self.strings, self.rows, new_m).unwrap()
    }

    pub fn sub_by_ref(&self, rhs: &Self) -> Self {
        if self.rows != rhs.rows || self.strings != rhs.strings {
            panic!("Matrices must have the same dimensions.")
        }

        let new_m: Vec<f64> = self.body
            .iter()
            .enumerate()
            .map(|(i, e)| e - rhs.body[i])
            .collect();

        Self::new(self.strings, self.rows, new_m).unwrap()
    }

    pub fn transpose(&self) -> Self {
        let mut new_m = self.body.to_vec();
        for (i, e) in self.body.iter().enumerate(){
            new_m[i/self.rows + self.rows*(i % self.rows)] = *e
        }
        Self::new(self.strings, self.rows, new_m).unwrap()
    }

    pub fn inverse(&self) -> Option<Self> {
        if self.rows != self.strings { return None }

        let det = self.det();
        if det == 0.0 { return None }

        let mut ads = Vec::<f64>::new();

        self._cofactor_matrix(&mut ads);
        let size = ads.len().isqrt();

        Some(
            Self::new(size,size,ads.to_vec()).unwrap()
                .transpose()
                .mul_num(1.0 / det)
        )
    }

    pub fn mul_num(self, num: f64) -> Self {
        let new_m: Vec<f64> = self.body
            .into_iter()
            .map(|e| e * num)
            .collect();

        Self::new(self.strings, self.rows, new_m).unwrap()
    }

    pub fn cofactor_matrix(&self) -> Self {
        let mut ads = Vec::<f64>::new();
        self._cofactor_matrix(&mut ads);
        let size = ads.len().isqrt();
        Self::new(
            size,
            size,
            ads
        ).unwrap()
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
}
