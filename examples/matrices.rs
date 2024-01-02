use std::error::Error;
use xmath::matrices::Matrix;


fn main() -> Result<(), Box<dyn Error>> {

    let m = Matrix::new(5, 5, vec![
        0.0, 5.0, 6.0, 7.0, 1.0,
        1.0, 4.0, 5.0, 1.0, 1.0,
        0.0, 3.0, 1.0, 2.0, 2.0,
        0.0, 1.0, 7.0, 8.0, 6.0,
        0.0, 1.0, 4.0, 4.0, 7.0,
    ])?;
    println!("det = {}", m.det());
    // det = 156.0



    let m1 = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        1.0, 2.0, 3.0,
    ])?;
    let m2 = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        1.0, 2.0, 3.0,
        5.0, 2.0, 1.0,
    ])?;
    println!("m1 * m2 = {}", m1.mul_by_ref(&m2)?);
    println!("m1 * m2 = {}", m1 * m2);
    // m1 * m2 = Matrix (3x3) = [
    // 18 12 12
    // 12 12 39
    // 12 39 30
    // ]
    // m1 * m2 = Matrix (3x3) = [
    // 18 12 12
    // 12 12 39
    // 12 39 30
    // ]



    let m = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        1.0, 2.0, 3.0,
        5.0, 2.0, 1.0,
    ])?;
    println!("m * 2 = {}", m.mul_num(2.0));
    // m * 2 = Matrix (3x3) = [
    // 2 4 6
    // 4 6 2
    // 6 2 4
    // ]



    let m1 = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ])?;
    let m2 = Matrix::new(3, 3, vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ])?;
    println!("m1 + m2 = {}", m1.add_by_ref(&m2)?);
    println!("m1 + m2 = {}", m1 + m2);
    // m1 + m2 = Matrix (3x3) = [
    // 2 3 5
    // 3 5 7
    // 5 7 1
    // ]
    // m1 + m2 = Matrix (3x3) = [
    // 2 3 5
    // 3 5 7
    // 5 7 1
    // ]



    let m1 = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ])?;
    let m2 = Matrix::new(3, 3, vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ])?;
    println!("m1 - m2 = {}", m1.sub_by_ref(&m2)?);
    println!("m1 - m2 = {}", m1 - m2);
    // m1 - m2 = Matrix (3x3) = [
    // 0 1 1
    // 1 1 1
    // 1 1 9
    // ]
    // m1 - m2 = Matrix (3x3) = [
    // 0 1 1
    // 1 1 1
    // 1 1 9
    // ]



    let m = Matrix::new(3, 3, vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ])?;
    println!("Cofactor matrix of m = {}", m.cofactor_matrix().unwrap());
    // Cofactor matrix of m = Matrix (3x3) = [
    // -67 6 45
    // 6 45 6
    // 45 6 -4
    // ]



    let m = Matrix::new(3, 3, vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ])?;
    println!("Transposed m = {}", m.transpose());
    // Transposed m = Matrix (3x3) = [
    // 1 3 6
    // 3 6 1
    // 6 1 -4
    // ]



    let m = Matrix::new(3, 3, vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ])?;
    let im = m.inverse().unwrap();
    println!("im = {}", im);
    // im = Matrix (3x3) = [
    // -2.310344827586207 0.20689655172413793 0.4482758620689655
    // 0.20689655172413793 0.4482758620689655 0.20689655172413793
    // 0.4482758620689655 0.20689655172413793 -0.13793103448275862
    // ]



    let m = Matrix::new(3, 3, vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ])?;
    println!("m^3 = {}", m.pow(3)?);
    // m = Matrix (3x3) = [
    // 187 133 271
    // 133 271 339
    // 271 339 -13
    // ]



    // SLAE (Kramer's method)
    let m = Matrix::new(3, 3, vec![
        1.0, 4.0, 2.0,
        2.0, -6.0, -2.0,
        1.0, 5.0, 2.0,
    ])?;
    let d = &[1.0, 3.0, 2.0];
    for (i, x) in m.slae(d)?.unwrap().iter().enumerate() {
        println!("x{} = {}", i+1, x);
    }
    // x1 = 2.0
    // x2 = 1.0
    // x3 = -2.5



    // Matrix equation
    let m1 = Matrix::new(2, 2, vec![
        2.0, 1.0,
        3.0, 2.0,
    ])?;
    let m2 = Matrix::new(2, 2, vec![
        0.0, 1.0,
        3.0, 2.0,
    ])?;
    let m3 = Matrix::new(2, 2, vec![
        1.0, -1.0,
        2.0, 1.0,
    ])?;
    println!("(m1 * x) + (m2 * 2) - m3 = 0");
    println!("x = {}", (m3 - m2.mul_num(2.0)) * m1.inverse().unwrap() );
    // (m1 * x) + (m2 * 2) - m3 = 0
    // x = Matrix (2x2) = [
    // 11 -7
    // -7 1
    // ]



    // Matrix equation
    let m1 = Matrix::new(2, 2, vec![
        10.0, 2.0,
        3.0, 1.0,
    ])?;
    let m2 = Matrix::new(2, 2, vec![
        2.0, 1.0,
        2.0, 3.0,
    ])?;
    let m3 = Matrix::new(2, 2, vec![
        1.0, 5.0,
        2.0, 4.0,
    ])?;
    println!("m1 * x * m2 = m3");
    println!("x = {}", m1.inverse().unwrap() * m3 * m2.inverse().unwrap() );
    // m1 * x * m2 = m3
    // x = Matrix (2x2) = [
    // -0.1875 -0.1875
    // -0.1875 0.0625
    // ]


    Ok(())
}
