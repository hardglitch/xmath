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
    println!("result = {:?}", m1 * m2);
    // result = Matrix { strings: 3, rows: 3, body: [18.0, 12.0, 12.0, 39.0, 30.0, 33.0, 18.0, 12.0, 12.0] }



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
    println!("result = {:?}", m1 + m2);
    // result = Matrix { strings: 3, rows: 3, body: [2.0, 3.0, 5.0, 7.0, 1.0, 11.0, 13.0, 15.0, 17.0] }
}
