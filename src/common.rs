use std::error::Error;
use crate::matrices::Matrix;
use crate::utils::AdvancedEQ;


pub fn factorial(n: usize) -> f64 {
    //! n! = 2 * 3 * ... * n
    //! Limit 170, if > then 'inf'

    match n {
        0 | 1 => { 1.0 }
        _ => {
            let mut r = Box::<f64>::new(1e0);
            for i in 2..=n {
                *r *= i as f64
            }
            *r
        }
    }
}


pub fn simple_sigma(n: usize) -> usize {
    let mut sum = 0;
    let mut value = 1;
    while value <= n {
        sum += value;
        value += 1;
    }
    sum
}


pub fn sigma<F>(first: f64, last: f64, step: f64, func: F) -> Result<f64, Box<dyn Error>>
    where F: Fn(f64) -> f64
{
    //! first - first value
    //!
    //! last - last value
    //!
    //! step - change step
    //!
    //! func - function, f(value)

    if (first-last).abs() < step.abs() { return Err("'Step' must be less than".into()); }
    if first.is_equal(last, 0.01) { return Err("'First' and 'last' must not be equal".into()); }
    if (first < last) && (step < 0.0) { return Err("If 'first' is less than 'last' then 'step' must be greater than 0".into()); }
    if (first > last) && (step > 0.0) { return Err("If 'first' is greater than 'last' then 'step' must be less than 0".into()); }
    if step == 0.0 { return Err("'Step' must not be equal to 0".into()); }

    let mut sum = 0.0;
    let mut value = first;

    while step > 0.0 && value < last ||
          step < 0.0 && value > last ||
          value.is_equal(last, 0.01)
    {
        sum += func(value);
        value += step;
    }
    Ok(sum)
}

pub fn fib(step: usize) -> f64 {
    let m = Matrix::new(2, 2, vec![1., 1., 1., 0.]).unwrap();
    *m.pow(step - 1).unwrap().body.first().unwrap()
}
