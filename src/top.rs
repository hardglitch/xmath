use std::error::Error;
use crate::common::factorial;

pub fn binominal_coefficient(m: usize, n: usize) -> Result<f32, Box<dyn Error>> {
    //! C_n^m = n! / m! * (n - m)! , m < n

    if m > n { return Err("Argument 'n' must be greater then 'm'.".into()); }
    if m == n { return Err("Argument 'n' must not be equal to 'm'.".into()); }
    if m == 0 { return Err("Argument 'm' must be greater then 0.".into()); }

    let mut opt_n = Box::<u128>::new(1);
    for i in m+1..=n {
        *opt_n *= i as u128;
    }
    Ok(*opt_n as f32 / factorial(n - m) as f32 )
}


pub fn bernoulli(m: usize, n: usize, p: f32, q: f32) -> Result<f32, Box<dyn Error>> {
    //! P_n^m = C_n^m * p^m * q^(n-m) , m < n
    //!
    //! m - required number of tests
    //!
    //! n - total number of tests
    //!
    //! p - probability of event occurrence in each test
    //!
    //! q - probability that the event will not occur in each test

    match crate::top::binominal_coefficient(m, n) {
        Ok(c) => Ok(c * p.powf(m as f32) * q.powf((n - m) as f32)),
        Err(e) => Err(e)
    }
}
