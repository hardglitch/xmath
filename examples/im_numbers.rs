use std::error::Error;
use xmath::im::cast::ImValue;
use xmath::im::im_matrices::ImMatrix;


fn main() -> Result<(), Box<dyn Error>> {

    // Import ImValue to implement im-power!
    //
    // .i() - imaginary number
    // .r() - real number
    //
    // Use standard operators such as +, -, *, /.


    // 3i (1 - i)^3i / (i^2 - 1)^2
    let expr = 3.i() * (1.r() - 1.i()).pow(3.i()) / (1.i().powi(2.r()) - 1.r()).pow(2.r());
    println!("{}", expr);
    // 0.75i(1-i)^3i


    // ImMatrix - Im version of the Matrix module.
    let m1 = ImMatrix::new(2, 2, vec![
        (-1).r() - 3.i(), 4.r() + 2.i(),
        1.r() + 1.i(), (-2).r() + 1.i(),
    ])?;

    let m2 = ImMatrix::new(2, 2, vec![
       (-5).r() - 3.i(), 4.r() + 2.i(),
       4.r() + 3.i(), (-8).r() + 1.i(),
    ])?;

    let d = &[1.r() - 1.i(), 2.r() + 1.i()];

    println!("det = {}", m1.det());
    // det = (3-i)

    println!("m1 * m2 = {}", m1.mul_by_ref(&m2)?);
    println!("m1 * m2 = {}", m1.clone() * m2.clone());
    // m1 * m2 = Matrix (2x2) = [
    //  (6+38i) (-32-26i)
    //  (-32-26i) (-13-10i)
    // ]
    // m1 * m2 = Matrix (2x2) = [
    //  (6+38i) (-32-26i)
    //  (-32-26i) (-13-10i)
    // ]

    println!("m * 2 = {}", m1.mul_num(2.r()));
    // m * 2 = Matrix (2x2) = [
    //  (-6i-2) (4i+8)
    //  (4i+8) (2i+2)
    // ]

    println!("m2^3 = {}", m2.pow(3)?);
    // m2^3 = Matrix (2x2) = [
    //  (-608i-70) (422+476i)
    //  (422+476i) (369+608i)
    // ]

    println!("m1 + m2 = {}", m1.add_by_ref(&m2)?);
    println!("m1 + m2 = {}", m1.clone() + m2.clone());
    // m1 + m2 = Matrix (2x2) = [
    //  (-6i-6) (4i+8)
    //  (4i+8) (4i+5)
    // ]
    // m1 + m2 = Matrix (2x2) = [
    //  (-6i-6) (4i+8)
    //  (4i+8) (4i+5)
    // ]

    println!("m1 - m2 = {}", m1.sub_by_ref(&m2)?);
    println!("m1 - m2 = {}", m1.clone() - m2.clone());
    // m1 - m2 = Matrix (2x2) = [
    // 4 0
    // 0 (-2i-3)
    // ]
    // m1 - m2 = Matrix (2x2) = [
    // 4 0
    // 0 (-2i-3)
    // ]

    println!("Cofactor matrix of m = {}", m1.cofactor_matrix().unwrap());
    // Cofactor matrix of m = Matrix (2x2) = [
    //  (i-2) (i+1)
    //  (i+1) (-2i-4)
    // ]

    println!("Transposed m = {}", m1.transpose());
    // Transposed m = Matrix (2x2) = [
    //  (-1-3i) (1+i)
    //  (1+i) (4+2i)
    // ]

    println!("im = {}", m1.inverse().unwrap());
    // im = Matrix (2x2) = [
    //  (i-2)/(3-i) (-2i-4)/(3-i)
    //  (-2i-4)/(3-i) (i+1)/(3-i)
    // ]

    for (i, x) in m1.slae(d)?.unwrap().iter().enumerate() {
        println!("x{} = {}", i+1, x);
    }
    // x1 = (-7-5i)/(3-i)
    // x2 = (-1-7i)/(3-i)


    Ok(())
}
