use std::error::Error;
use crate::common::factorial;

pub fn binominal_coefficient(m: usize, n: usize) -> Result<f64, Box<dyn Error>> {
    //! C_n^m = n! / m! * (n - m)! , m < n

    if m > n { return Err("Argument 'n' must be greater then 'm'.".into()); }
    if m == n { return Err("Argument 'n' must not be equal to 'm'.".into()); }
    if m == 0 { return Err("Argument 'm' must be greater then 0.".into()); }

    let mut opt_n = Box::<f64>::new(1e0);
    for i in m+1..=n {
        *opt_n *= i as f64;
    }
    Ok(*opt_n / factorial(n - m))
}


pub fn bernoulli(m: usize, n: usize, p: f64, q: f64) -> Result<f64, Box<dyn Error>> {
    //! P_n^m = C_n^m * p^m * q^(n-m) , m < n
    //!
    //! m - required number of tests
    //!
    //! n - total number of tests
    //!
    //! p - probability of event occurrence in each test
    //!
    //! q - probability that the event will not occur in each test

    match binominal_coefficient(m, n) {
        Ok(c) => Ok(c * p.powf(m as f64) * q.powf((n - m) as f64)),
        Err(e) => Err(e)
    }
}
