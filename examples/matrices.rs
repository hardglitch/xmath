use xmath::matrices::Matrix;

fn main() {
    let m = Matrix::new(5, 5, vec![
        0.0, 5.0, 6.0, 7.0, 1.0,
        1.0, 4.0, 5.0, 1.0, 1.0,
        0.0, 3.0, 1.0, 2.0, 2.0,
        0.0, 1.0, 7.0, 8.0, 6.0,
        0.0, 1.0, 4.0, 4.0, 7.0,
    ]).unwrap();

    println!("det = {:?}", m.det());
    // det = 156.0



    let m1 = Matrix::new(3,3,vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        1.0, 2.0, 3.0,
    ]).unwrap();
    let m2 = Matrix::new(3,3,vec![
        1.0, 2.0, 3.0,
        1.0, 2.0, 3.0,
        5.0, 2.0, 1.0,
    ]).unwrap();
    println!("m1 * m2 = {:?}", m1 * m2);
    // m1 * m2 = Matrix { strings: 3, rows: 3, body: [18.0, 12.0, 12.0, 39.0, 30.0, 33.0, 18.0, 12.0, 12.0] }



    let m = Matrix::new(3,3,vec![
        1.0, 2.0, 3.0,
        1.0, 2.0, 3.0,
        5.0, 2.0, 1.0,
    ]).unwrap();
    println!("m * 2 = {:?}", m.mul_num(2.0));
    // m * 2 = Matrix { strings: 3, rows: 3, body: [2.0, 4.0, 6.0, 2.0, 4.0, 6.0, 10.0, 4.0, 2.0] }



    let m1 = Matrix::new(3,3,vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ]).unwrap();
    let m2 = Matrix::new(3,3,vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ]).unwrap();
    println!("m1 + m2 = {:?}", m1 + m2);
    // m1 + m2 = Matrix { strings: 3, rows: 3, body: [2.0, 3.0, 5.0, 7.0, 1.0, 11.0, 13.0, 15.0, 17.0] }



    let m = Matrix::new(3,3,vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ]).unwrap();
    println!("Inverse m = {:?}", m.inverse().unwrap());
    // Inverse m = Matrix { strings: 3, rows: 3, body: [-2.310344827586207, 0.20689655172413793,
    // 0.4482758620689655, 0.20689655172413793,-0.13793103448275862, 0.034482758620689655,
    // 1.5517241379310345, -0.034482758620689655, -0.24137931034482757] }



    let m = Matrix::new(3,3,vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ]).unwrap();
    println!("Cofactor matrix of m = {:?}", m.cofactor_matrix());
    // Cofactor matrix of m = Matrix { strings: 3, rows: 3, body: [-67.0, 6.0, 45.0, 6.0, -4.0, -1.0, 13.0, 1.0, -7.0] }



    let m = Matrix::new(3,3,vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ]).unwrap();
    println!("Transposed m = {:?}", m.transpose());
    // Transposed m = Matrix { strings: 3, rows: 3, body: [1.0, 3.0, 6.0, 1.0, -4.0, 7.0, 2.0, 5.0, 8.0] }



    let m = Matrix::new(3,3,vec![
        1.0, 1.0, 2.0,
        3.0, -4.0, 5.0,
        6.0, 7.0, 8.0,
    ]).unwrap();
    let im = m.inverse().unwrap();
    println!("m * im = im * m: {:?}", m.mul_by_ref(&im) == im.mul_by_ref(&m));
    println!("m * im = {:?}", m * im);
    // m * im = im * m: true
    // m * im = Matrix { strings: 3, rows: 3, body: [1.0, 0.0, 5.551115123125783e-17, 0.0, 1.0, 0.0, 0.0, 1.1102230246251565e-16, 1.0] }
    // 1
    //   1       - E
    //     1
}
