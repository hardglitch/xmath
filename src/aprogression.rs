use crate::common::sigma;

pub fn sum(a1: f32, d: f32, n: usize) -> f32 {
    //! sum = a1 * n + d * sigma(n - 1)

    a1 * n as f32 + d * sigma(n - 1) as f32
}

pub fn get_n(a1: f32, d: f32, sum: f32) -> usize {
    //! n - number of iterations will be spent before 'sum' is achieved

    let mut n = Box::<usize>::new(1);
    while crate::aprogression::sum(a1, d, *n) < sum {
        *n += 1
    }
    *n
}

pub fn get_a_n(a_k: f32, d: f32, k: usize, n: usize) -> f32 {
    //! a_n = a_k + (n - k) * d , n => k
    //!
    //! a_n = a_k - (k - n) * d , n <= k

    if n > k { return a_k + (n - k) as f32 * d }
    a_k - (k - n) as f32 * d
}
