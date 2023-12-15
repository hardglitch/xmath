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
    println!("det = {:?}", m.det());
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
    println!("m1 * m2 = {:?}", m1.mul_by_ref(&m2)?);
    println!("m1 * m2 = {:?}", m1 * m2);
    // m1 * m2 = Matrix { strings: 3, rows: 3, body: [18.0, 12.0, 12.0, 39.0, 30.0, 33.0, 18.0, 12.0, 12.0] }
    // m1 * m2 = Matrix { strings: 3, rows: 3, body: [18.0, 12.0, 12.0, 39.0, 30.0, 33.0, 18.0, 12.0, 12.0] }



    let m = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        1.0, 2.0, 3.0,
        5.0, 2.0, 1.0,
    ])?;
    println!("m * 2 = {:?}", m.mul_num(2.0));
    // m * 2 = Matrix { strings: 3, rows: 3, body: [2.0, 4.0, 6.0, 2.0, 4.0, 6.0, 10.0, 4.0, 2.0] }



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
    println!("m1 + m2 = {:?}", m1.add_by_ref(&m2)?);
    println!("m1 + m2 = {:?}", m1 + m2);
    // m1 + m2 = Matrix { strings: 3, rows: 3, body: [2.0, 3.0, 5.0, 7.0, 1.0, 11.0, 13.0, 15.0, 17.0] }
    // m1 + m2 = Matrix { strings: 3, rows: 3, body: [2.0, 3.0, 5.0, 7.0, 1.0, 11.0, 13.0, 15.0, 17.0] }



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
    println!("m1 - m2 = {:?}", m1.sub_by_ref(&m2)?);
    println!("m1 - m2 = {:?}", m1 - m2);
    // m1 - m2 = Matrix { strings: 3, rows: 3, body: [0.0, 1.0, 1.0, 1.0, 9.0, 1.0, 1.0, 1.0, 1.0] }
    // m1 - m2 = Matrix { strings: 3, rows: 3, body: [0.0, 1.0, 1.0, 1.0, 9.0, 1.0, 1.0, 1.0, 1.0] }



    let m = Matrix::new(3, 3, vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ])?;
    println!("Cofactor matrix of m = {:?}", m.cofactor_matrix().unwrap());
    // Cofactor matrix of m = Matrix { strings: 3, rows: 3, body: [-67.0, 6.0, 45.0, 6.0, -4.0, -1.0, 13.0, 1.0, -7.0] }



    let m = Matrix::new(3, 3, vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ])?;
    println!("Transposed m = {:?}", m.transpose());
    // Transposed m = Matrix { strings: 3, rows: 3, body: [1.0, 3.0, 6.0, 1.0, -4.0, 7.0, 2.0, 5.0, 8.0] }



    let m = Matrix::new(3, 3, vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ])?;
    let im = m.inverse().unwrap();
    println!("m * im = im * m: {:?}", m.mul_by_ref(&im)? == im.mul_by_ref(&m)?);
    // or
    // println!("m * im = im * m: {:?}", m.clone() * im.clone() == im.clone() * m.clone());
    println!("m * im = {:?}", m * im);
    // m * im = im * m: true
    // m * im = Matrix { strings: 3, rows: 3, body: [1.0, 0.0, 5.551115123125783e-17, 0.0, 1.0, 0.0, 0.0, 1.1102230246251565e-16, 1.0] }
    // 1
    //   1       - identity matrix
    //     1



    let m = Matrix::new(3, 3, vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ])?;
    let m1 = m.pow(3)?;
    let m2 = m.mul_by_ref(&m)?.mul_by_ref(&m)?;
    let m3 = m.clone() * m.clone() * m;
    println!("m1 = {:?}", m1);
    println!("m2 = {:?}", m2);
    println!("m3 = {:?}", m3);
    // m1 = Matrix { strings: 3, rows: 3, body: [187.0, 133.0, 271.0, 339.0, -13.0, 520.0, 843.0, 716.0, 1208.0] }
    // m2 = Matrix { strings: 3, rows: 3, body: [187.0, 133.0, 271.0, 339.0, -13.0, 520.0, 843.0, 716.0, 1208.0] }
    // m3 = Matrix { strings: 3, rows: 3, body: [187.0, 133.0, 271.0, 339.0, -13.0, 520.0, 843.0, 716.0, 1208.0] }



    // SLAE (Kramer's method)
    let m = Matrix::new(3, 3, vec![
        1.0, 4.0, 2.0,
        2.0, -6.0, -2.0,
        1.0, 5.0, 2.0,
    ])?;
    let d = &[1.0, 3.0, 2.0];
    for (i, x) in m.slae(d)?.unwrap().iter().enumerate() {
        println!("x{} = {:?}", i+1, x);
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
    println!("x = {:?}", (m3 - m2.mul_num(2.0)) * m1.inverse().unwrap() );
    // (m1 * x) + (m2 * 2) - m3 = 0
    // x = Matrix { strings: 2, rows: 2, body: [11.0, -7.0, 1.0, -2.0] }



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
    println!("x = {:?}", m1.inverse().unwrap() * m3 * m2.inverse().unwrap() );
    // m1 * x * m2 = m3
    // x = Matrix { strings: 2, rows: 2, body: [-0.1875, -0.1875, 0.0625, 2.0625] }



    Ok(())
}
