use xmath::aprogression::{get_a_n, get_d, get_iters, sum_by_elems};

fn main() {
    let d = get_d(-3.0, 8, 91.0, 4);
    println!("{:?}", d.ok());
    // Some(-23.5)

    let iters = get_iters(60.0, 17.0, 1000.0).unwrap();
    println!("{:?}", iters);
    // 9

    let a_n = get_a_n(3.0, 2, 6, 2.0).unwrap();
    println!("{:?}", a_n);
    // 11.0

    let sum = sum_by_elems(0.0, 8, -8.0, 3).unwrap();
    println!("{:?}", sum);
    // 24.0
}