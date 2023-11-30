use std::error::Error;

pub fn sum(b1: f64, q: f64, n: usize) -> Result<f64, Box<dyn Error>> {
    //! sum = b1 * (q^n - 1) / (q - 1)

    if q == 1.0 { return Err("Argument 'q' must not be 1.0".into()) }
    Ok(b1 * (q.powf(n as f64) - 1.0) / (q - 1.0))
}

pub fn get_n(b1: f64, q: f64, sum: f64) -> usize {
    //! 'n' - number of iterations will be spent before 'sum' is achieved

    let mut n = Box::<usize>::new(1);
    while crate::gprogression::sum(b1, q, *n).ok() < Some(sum) {
        *n += 1
    }
    *n
}

pub fn get_b_n(b_k: f64, q: f64, k: usize, n: usize) -> Result<f64, Box<dyn Error>> {
    //! b_n = b_k * q^(n - k) , n > k
    //!
    //! b_n = b_k / q^(k - n) , n < k
    //!
    //! b_k - value of the known element
    //!
    //! q - ratio of progression
    //!
    //! k - index of the known element
    //!
    //! n - index of the required element

    if q == 0.0 { return Err("Argument 'q' must not be 0".into()) }

    if n > k { return Ok(b_k * q.powf((n - k) as f64)) }
    Ok(b_k / q.powf((k - n) as f64))
}

pub fn get_q(b_k: f64, b_n: f64, k: usize, n: usize) -> Result<f64, Box<dyn Error>> {
    //! q = (b_n / b_k)^(1 / (n - k)) , n > k
    //!
    //! q = (b_k / b_n)^(1 / (k - n)) , n < k
    //!
    //! b_k - value of the first known element
    //!
    //! b_n - value of the second known element
    //!
    //! k - index of the first known element
    //!
    //! n - index of the second known element

    if n == k { return Err("Argument 'n' must not be equal to 'k'.".into()) }

    if n > k {
        if b_k == 0.0 { return Err("Argument 'b_k' must not be 0".into()) }
        return Ok((b_n / b_k).powf(1.0 / (n - k) as f64))
    }
    if b_n == 0.0 { return Err("Argument 'b_n' must not be 0".into()) }
    Ok((b_k / b_n).powf(1.0 / (k - n) as f64))
}
