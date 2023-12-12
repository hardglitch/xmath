use std::error::Error;
use std::fmt::{Display, Formatter};
use std::mem::swap;
use std::ops::{Add, Mul, Sub};


#[derive(Debug)]
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
            self.body.eq(&other.body)
    }
}
impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
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

        Matrix::new(left.strings, right.rows, new_m).unwrap()
    }
}
impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.strings != rhs.strings {
            panic!("Matrices must have the same dimensions.")
        }

        let new_m: Vec<f64> = self.body
            .into_iter()
            .enumerate()
            .map(|(i, e)| e + rhs.body[i])
            .collect();

        Matrix::new(self.strings, self.rows, new_m).unwrap()
    }
}
impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.strings != rhs.strings {
            panic!("Matrices must have the same dimensions.")
        }

        let new_m: Vec<f64> = self.body
            .into_iter()
            .enumerate()
            .map(|(i, e)| e - rhs.body[i])
            .collect();

        Matrix::new(self.strings, self.rows, new_m).unwrap()
    }
}
impl Matrix {
    pub fn new(strings: usize, rows: usize, body: Vec<f64>) -> Result<Self, Box<dyn Error>> {
        if strings == 0 || rows == 0 { return Err("Arguments must be greater than 0.".into()) }
        if body.len() != strings * rows { return Err("The matrix have an incorrect size.".into()) }

        Ok(Self { strings, rows, body })
    }

    pub fn det(&self) -> f64
    {
        if self.rows != self.strings { return 0.0 }

        let mut d = 0_f64;
        if self.body.len() > 4 {
            for r in 0..self.rows {
                let elem = self.body[r * self.rows];
                let ad = (-1_f64).powi((1 + r+1) as i32);
                let str = (r * self.rows..r * self.rows + self.rows).collect::<Vec<usize>>();

                let sub_matrix: Vec<f64> = self.body
                    .iter()
                    .enumerate()
                    .filter_map(|(i, e)|
                        if (i % self.rows != 0) && !str.iter().any(|x| *x == i) {Some(*e)} else {None}
                    )
                    .collect();
                let size = sub_matrix.len().isqrt();
                let new_m = Matrix::new(
                    size,
                    size,
                    sub_matrix
                ).unwrap();
                d += elem * ad * new_m.det();
            }
        } else {
            d += self.body[0]*self.body[3] - self.body[1]*self.body[2];
        }
        d
    }

    pub fn transpose(&self) -> Matrix {
        let mut new_m = self.body.to_vec();
        for (i, e) in self.body.iter().enumerate(){
            new_m[i/self.rows + self.rows*(i % self.rows)] = *e
        }
        Matrix::new(self.strings, self.rows, new_m).unwrap()
    }
}
