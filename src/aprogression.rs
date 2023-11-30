use std::error::Error;
use crate::common::simple_sigma;

pub fn sum_by_d(a_k: f64, k: usize, n: usize, d: f64) -> Result<f64, Box<dyn Error>> {
    //! sum of the arithmetic progression
    //!
    //! S = a_k * (n - k + 1) + d * (1 + 2 + .. + (n - k)) , n > k
    //!
    //! S = a_k * (k - n + 1) + d * (1 + 2 + .. + (k - n)) , n < k
    //!
    //! a_k - value of the known element
    //!
    //! k - index of the known element
    //!
    //! n - index of the required element
    //!
    //! d - arithmetical ratio (step)

    if n == k { return Err("Argument 'n' must not be equal to 'k'.".into()); }

    if n < k {
        return Ok(a_k * (k - n + 1) as f64 + d * simple_sigma(k - n) as f64)
    }
    Ok(a_k * (n - k + 1) as f64 + d * simple_sigma(n - k) as f64)
}


pub fn sum_by_elems(a_k: f64, k: usize, a_n: f64, n: usize) -> Result<f64, Box<dyn Error>> {
    //! Arithmetic progression sum
    //!
    //! S = a_n * (n - k + 1) + ((a_n - a_k) / (n - k)) * (1 + 2 + .. + (n - k)) , n > k
    //!
    //! S = a_k * (k - n + 1) + ((a_k - a_n) / (k - n)) * (1 + 2 + .. + (k - n)) , n < k
    //!
    //! a_k - value of the first known element
    //!
    //! k - index of the first known element
    //!
    //! a_n - value of the second known element
    //!
    //! n - index of the second known element

    if n == k { return Err("Argument 'n' must not be equal to 'k'.".into()); }

    if n < k {
        return Ok(a_k * (k - n + 1) as f64 + ((a_k - a_n) / (k - n) as f64) * simple_sigma(k - n) as f64)
    }
    Ok(a_n * (n - k + 1) as f64 + ((a_n - a_k) / (n - k) as f64) * simple_sigma(n - k) as f64)
}


pub fn get_d(a_k: f64, k: usize, a_n: f64, n: usize) -> Result<f64, Box<dyn Error>> {
    //! Arithmetic progression ratio (step)
    //!
    //! d = ((a_n - a_k) / (n - k)), n > k
    //!
    //! d = ((a_k - a_n) / (k - n)), n < k
    //!
    //! a_k - value of the first known element
    //!
    //! k - index of the first known element
    //!
    //! a_n - value of the second known element
    //!
    //! n - index of the second known element

    if n == k { return Err("Argument 'n' must not be equal to 'k'.".into()); }

    if n < k {
        return Ok((a_k - a_n) / (k - n) as f64)
    }
    Ok((a_n - a_k) / (n - k) as f64)
}


pub fn get_iters(base: f64, d: f64, sum: f64) -> Result<usize, Box<dyn Error>> {
    //! Number of iterations will be spent before 'sum' is achieved
    //!
    //! a1 - the first element of progression
    //!
    //! d - arithmetical ratio (step)
    //!
    //! sum - arithmetic progression sum

    if base < 0.0 { return Err("Arguments 'base' must not be less than 0".into()) }
    if sum <= 0.0 || d <= 0.0 { return Err("Arguments 'd' and 'sum' must be greater than 0".into()) }

    let mut n = Box::<usize>::new(1);
    while let s = sum_by_d(base, 1, *n, d) && s.ok() < Some(sum) {
        *n += 1
    }
    Ok(*n)
}

pub fn get_a_n(a_k: f64, k: usize, n: usize, d: f64) -> Result<f64, Box<dyn Error>> {
    //! Get value of element 'n' in progression
    //!
    //! a_n = a_k + (n - k) * d , n > k
    //!
    //! a_n = a_k - (k - n) * d , n < k
    //!
    //! a_k - value of the known element
    //!
    //! k - index of the known element
    //!
    //! n - index of the required element
    //!
    //! d - arithmetical ratio (step)

    if n == k { return Err("Argument 'n' must not be equal to 'k'.".into()); }

    if n > k { return Ok(a_k + (n - k) as f64 * d) }
    Ok(a_k - (k - n) as f64 * d)
}
