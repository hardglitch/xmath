use xmath::im_numbers::cast::ImValue;

fn main() {
    // (1 + 2i)(3 - i) = ?   =>  (2i^1 + 1)(-1i^1 + 3)
    // (1 + 3i^2 - 2i)(3 - i) = ?   =>  (3i^2 - 2i^1 + 1)(-1i^1 + 3)
    // let expr = 0.r() * (-2).r(); // 0
    // let expr = 1.0.cast() + i_2i * 2.0.cast(); // 1 + 4i
    // let expr = 1.0.cast() - i_2i * 2.0.cast(); // 1 - 4i
    // let expr = (1.0.cast() + i_2i) * 3.0.cast(); // 3 + 6i
    // let expr = (1.0.cast() + i_2i) * (i_1i.clone() + i_1i); // 2i - 4
    // let expr = (2.i() + 1.r()) * (3.r() + 1.i()); // 7i - 1
    // let expr = (1.r() + 2.i()) * (3.r() + 1.i()) + 4.i() - (4.i() + 6.r()); // 7i - 5
    // let expr = 1.r() * (2.i() - 2.i()) * 5.r() - (2.i() + 5.r()) * 2.i(); // 4 - 10i
    let expr = 1.i() / 1.i();
    // dbg!(expr);
    println!("{}", expr);
}
