use xmath::im::cast::ImValue;

fn main() {
    // 3i (1 - i)^3i / (i^2 - 1)^2
    let expr = 3.i() * (1.r() - 1.i()).pow(3.i()) / (1.i().powi(2.r()) - 1.r()).pow(2.r());
    // 0.75i(1-i)^3i
    println!("{}", expr);
}
