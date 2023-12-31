use std::error::Error;
use xmath::algebra::quadratic_equation;


fn main() -> Result<(), Box<dyn Error>> {
    // 5x^2 - 63x - 18 = 0
    let res = quadratic_equation(5.0, -63.0, -18.0)?.unwrap();
    println!("x1 = {:.2?}", &res.first().unwrap());
    println!("x2 = {:.2?}", &res.last().unwrap());
    // x1 = 12.88
    // x2 = -0.99

    Ok(())
}