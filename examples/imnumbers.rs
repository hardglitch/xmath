use xmath::im_numbers::{Im, XValue};

fn main() {

    // (1 + 2i)(3 - i) = ?   =>  (2i^1 + 1)(-1i^1 + 3)
    // (1 + 3i^2 - 2i)(3 - i) = ?   =>  (3i^2 - 2i^1 + 1)(-1i^1 + 3)
    let i2 = Im::new("2i").unwrap();
    let i1 = Im::new("1i").unwrap();
    let i4 = Im::new("4i").unwrap();
    let expr = (1.0.cast() + i2) + (3.0.cast() + i1) + i4.clone() - (i4 + 6.0.cast());
    // let expr = (1.0.cast() + i2 + 2.0.cast());
    // dbg!(expr);
    println!("{}", expr);
}
