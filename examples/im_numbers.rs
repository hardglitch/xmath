use xmath::im::cast::ImValue;

fn main() {

    // Import ImValue to implement im-power!
    //
    // .i() - imaginary number
    // .r() - real number
    //
    // Use standard operators such as +, -, *, /.

    // 3i (1 - i)^3i / (i^2 - 1)^2
    let expr = 3.i() * (1.r() - 1.i()).pow(3.i()) / (1.i().powi(2.r()) - 1.r()).pow(2.r());
    println!("{}", expr);
    // 0.75i(1-i)^3i
}
