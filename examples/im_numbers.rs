use xmath::im_numbers::cast::ImValue;

fn main() {
    // (1 + 2i)(3 - i) = ?   =>  (2i^1 + 1)(-1i^1 + 3)
    // (1 + 3i^2 - 2i)(3 - i) = ?   =>  (3i^2 - 2i^1 + 1)(-1i^1 + 3)
    let expr = 1.r() + 1.r();
    // let expr = 0.r() * (-2).r(); // 0
    // let expr = (2.i() + 1.r()) * (3.r() + 1.i()); // 7i - 1
    // let expr = (1.r() + 2.i()) * (3.r() + 1.i()) + 4.i() - (4.i() + 6.r()); // 7i - 5
    // let expr = 1.r() * (2.i() - 2.i()) * 5.r() - (2.i() + 5.r()) * 2.i(); // 4 - 10i
    // let expr = 2.r() / 1.i();
    // let expr = (4.r() - 7.i()) / (6.i() - 7.i()); // 7 - 4/i
    // let expr = (4.r() - 7.i()) / (6.r() - 7.i()); // (4-7i)/(6-7i)
    // let expr = (1.r() - 1.i()) / (1.i() - 1.r()); //
    // dbg!(expr);
    println!("{}", expr);
}
