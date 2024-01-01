use xmath::gprogression::{get_b_n, get_iters, get_q, sum};

fn main() {
    let d = get_q(3.0, 80.0, 2, 4);
    println!("{:.2?}", d.ok());
    // Some(5.16)

    let iters = get_iters(60.0, 17.0, 1000.0).unwrap();
    println!("{:?}", iters);
    // 2

    let a_n = get_b_n(3.0, 2.0, 6, 2).unwrap();
    println!("{:?}", a_n);
    // 0.1875

    let sum = sum(0.8, 0.1, 1).unwrap();
    println!("{:?}", sum);
    // 0.8
}