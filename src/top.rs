use std::error::Error;
use crate::common::factorial;

#[allow(dead_code)]
pub fn binominal_coefficient(m: usize, n: usize) -> Result<f64, Box<dyn Error>> {
    //! C_n^m = n! / m! * (n - m)!

    if n < m { return Err("Argument 'n' must be greater then 'm'.".into()); }

    let mut opt_n = Box::<u128>::new(1);
    for i in m+1..=n {
        *opt_n *= i as u128;
    }
    let dif = n - m;
    Ok(*opt_n as f64 / factorial(dif) as f64 )
}


#[allow(dead_code)]
pub fn bernoulli(m: usize, n: usize, p: f64, q: f64) -> Result<f64, Box<dyn Error>> {
    //! P_n^m = C_n^m * p^m * q^(n-m)

    match crate::top::binominal_coefficient(m, n) {
        Ok(c) => Ok(c * p.powf(m as f64) * q.powf((n - m) as f64)),
        Err(e) => Err(e)
    }
}
