use std::error::Error;


#[allow(dead_code)]
pub fn factorial(n: &usize) -> u128 {
    //! n! = 2 * 3 * ... * n

    match *n {
        0 => { 0 }
        1 => { 1 }
        _ => {
            let mut r = Box::<u128>::new(1);
            for i in 2..=*n as u128 {
                *r *= i
            }
            *r
        }
    }
}


#[allow(dead_code)]
pub fn binominal_coefficient(m: &usize, n: &usize) -> Result<f64, Box<dyn Error>> {
    //! C_n^m = n! / m! * (n - m)!

    if n < m { return Err("Argument 'n' must be greater then 'm'.".into()); }

    let mut opt_n = Box::<u128>::new(1);
    for i in m+1..=*n {
        *opt_n *= i as u128;
    }
    let dif = n - m;
    Ok(*opt_n as f64 / factorial(&dif) as f64 )
}


#[allow(dead_code)]
pub fn bernoulli(m: &usize, n: &usize, p: &f64, q: &f64) -> Result<f64, Box<dyn Error>> {
    //! P_n^m = C_n^m * p^m * q^(n-m)

    match binominal_coefficient(m, n) {
        Ok(c) => Ok(c * p.powf(*m as f64) * q.powf((n - m) as f64)),
        Err(e) => Err(e)
    }
}

#[allow(dead_code)]
pub fn quadratic_equation(a: &f64, b: &f64, c: &f64) -> Result<Option<[f64; 2]>, Box<dyn Error>> {
    //! +-ax^2 +-bx +-с = 0

    if *a == 0.0 { return Err("Argument 'a' must not be 0.".into()); }

    let d = b.powf(2.0) - 4.0 * a * c;
    if d < 0.0 { return  Ok(None) }

    let x1 = (-b + d.sqrt()) / 2.0 * a;
    let x2 = (-b - d.sqrt()) / 2.0 * a;
    Ok(Some([x1, x2]))
}


#[allow(dead_code)]
pub fn sigma(n: &usize) -> usize {
    //! sigma(n) = 1 + 2 + 3 + ... + n

    let mut sum: usize = 0;
    for i in 1..=*n {
        sum += i;
    }
    sum
}



pub struct AProgression {}

impl AProgression {

    #[allow(dead_code)]
    pub fn sum(a1: &f64, d: &f64, n: &usize) -> f64 {
        //! sum = a1 * n + d * sigma(n - 1)

        a1 * *n as f64 + d * sigma(&(n - 1)) as f64
    }

    #[allow(dead_code)]
    pub fn get_n(a1: &f64, d: &f64, sum: &f64) -> usize {
        //! 'n' - number of iterations will be spent before 'sum' is achieved

        let mut n = Box::<usize>::new(1);
        while AProgression::sum(a1, d, &n) < *sum {
            *n += 1
        }
        *n
    }

    #[allow(dead_code)]
    pub fn get_a_n(a_k: &f64, d: &f64, k: &usize, n: &usize) -> f64 {
        //! a_n = a_k + (n - k) * d , n => k
        //!
        //! a_n = a_k - (k - n) * d , n <= k

        if n > k { return a_k + (n - k) as f64 * d }
        a_k - (k - n) as f64 * d
    }
}


pub struct GProgression {}

impl crate::math::GProgression {

    #[allow(dead_code)]
    pub fn sum(b1: &f64, q: &f64, n: &usize) -> Result<f64, Box<dyn Error>> {
        //! sum = b1 * (q^n - 1) / (q - 1)

        if *q == 1.0 { return Err("Argument 'q' must not be 1.0".into()) }
        Ok(b1 * (q.powf(*n as f64) - 1.0) / (q - 1.0))
    }

    #[allow(dead_code)]
    pub fn get_n(b1: &f64, q: &f64, sum: &f64) -> usize {
        //! 'n' - number of iterations will be spent before 'sum' is achieved

        let mut n = Box::<usize>::new(1);
        while crate::math::GProgression::sum(b1, q, &n).ok() < Some(*sum) {
            *n += 1
        }
        *n
    }

    #[allow(dead_code)]
    pub fn get_b_n(b_k: &f64, q: &f64, k: &usize, n: &usize) -> Result<f64, Box<dyn Error>> {
        //! b_n = b_k * q^(n - k) , n > k
        //!
        //! b_n = b_k / q^(k - n) , n < k
        //!
        //! b_k - value of a known element
        //!
        //! q - ratio of the progression
        //!
        //! k - index of a known element
        //!
        //! n - index of a required element

        if *q == 0.0 { return Err("Argument 'q' must not be 0".into()) }

        if n > k { return Ok(b_k * q.powf((n - k) as f64)) }
        Ok(b_k / q.powf((k - n) as f64))
    }

    #[allow(dead_code)]
    pub fn get_q(b_k: &f64, b_n: &f64, k: &usize, n: &usize) -> Result<f64, Box<dyn Error>> {
        //! q = (b_n / b_k)^(1 / (n - k)) , n > k
        //!
        //! q = (b_k / b_n)^(1 / (k - n)) , n < k
        //!
        //! b_k - value of first known element
        //!
        //! b_n - value of second known element
        //!
        //! k - index of first known element
        //!
        //! n - index of second known element

        if n == k { return Err("Argument 'n' must not be equal to 'k'.".into()) }

        if n > k {
            if *b_k == 0.0 { return Err("Argument 'b_k' must not be 0".into()) }
            return Ok((b_n / b_k).powf(1.0 / (n - k) as f64))
        }
        if *b_n == 0.0 { return Err("Argument 'b_n' must not be 0".into()) }
        Ok((b_k / b_n).powf(1.0 / (k - n) as f64))
    }
}

pub struct VectorAlgebra {}

#[derive(PartialEq, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl VectorAlgebra {
    #[allow(dead_code)]
    pub fn cos_alpha(a: &Vector3D, b: &Vector3D) -> Result<f64, Box<dyn Error>> {
        let ab = a.x * b.x + a.y * b.y + a.z * b.z;
        let a = (a.x.powf(2.0) + a.y.powf(2.0) + a.z.powf(2.0)).sqrt();
        let b = (b.x.powf(2.0) + b.y.powf(2.0) + b.z.powf(2.0)).sqrt();
        Ok(ab / (a * b))
    }

    #[allow(dead_code)]
    pub fn vector_multiplication(a: &Vector3D, b: &Vector3D) -> Vector3D {
        Vector3D{
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}
