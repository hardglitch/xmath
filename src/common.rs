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
pub fn sigma(n: &usize) -> usize {
    //! sigma(n) = 1 + 2 + 3 + ... + n

    let mut sum: usize = 0;
    for i in 1..=*n {
        sum += i;
    }
    sum
}
