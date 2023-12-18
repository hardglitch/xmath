use xmath::im_numbers::{Im, XValue};

fn main() {

    // (1 + 2i)(3 - i) = ?   =>  (2i^1 + 1)(-1i^1 + 3)
    // (1 + 3i^2 - 2i)(3 - i) = ?   =>  (3i^2 - 2i^1 + 1)(-1i^1 + 3)
    let i_2i = Im::new("2i").unwrap();
    let i_1i = Im::new("1i").unwrap();
    let i_4i = Im::new("4i").unwrap();
    // let expr = (1.0.cast() + i_2i) * (3.0.cast() + i_1i) + i_4i.clone() - (i_4i + 6.0.cast());
    // let expr = (1.0.cast() + i_2i * 2.0.cast());
    // let expr = (1.0.cast() - i_2i * 2.0.cast());
    let expr = 1.0.cast() - (i_2i + 5.0.cast());
    dbg!(expr);
    // println!("{}", expr);
}
