pub fn factorial(n: usize) -> f64 {
    //! n! = 2 * 3 * ... * n
    //! // Limit 170

    match n {
        0 => { 0.0 }
        1 => { 1.0 }
        _ => {
            let mut r = Box::<f64>::new(1e0);
            for i in 2..=n {
                *r *= i as f64
            }
            *r
        }
    }
}


pub fn sigma(n: usize) -> usize {
    //! sigma(n) = 1 + 2 + 3 + ... + n

    let mut sum: usize = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}
