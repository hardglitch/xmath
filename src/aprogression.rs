use crate::common::sigma;

#[allow(dead_code)]
pub fn sum(a1: &f64, d: &f64, n: &usize) -> f64 {
    //! sum = a1 * n + d * sigma(n - 1)

    a1 * *n as f64 + d * sigma(&(n - 1)) as f64
}

#[allow(dead_code)]
pub fn get_n(a1: &f64, d: &f64, sum: &f64) -> usize {
    //! 'n' - number of iterations will be spent before 'sum' is achieved

    let mut n = Box::<usize>::new(1);
    while crate::aprogression::sum(a1, d, &n) < *sum {
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
