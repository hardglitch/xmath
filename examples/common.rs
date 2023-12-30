use xmath::common::{factorial, sigma};

fn main() {
    let res = sigma(1.0, 10000.0, 1.0, |x| x);
    println!("{:?}", res);
    // Ok(50005000.0)

    let res = sigma(1.0, 1.1, 0.0001, |x| x);
    println!("{:.2?}", res);
    // Ok(1161.55)

    let a = factorial(170);
    println!("{:?}", a);
    // 7.257415615307994e306
}
