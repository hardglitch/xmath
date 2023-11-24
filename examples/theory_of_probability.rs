use math::top::bernoulli;


fn bernoulli_example() {
    let m = 2;
    let n = 5;
    let p = 1.0/6.0;
    let q = 1.0 - p;
    let res = bernoulli(m, n, p, q);
    println!("Result = {:.2?}", res.ok().unwrap());
    // Result = 0.16
}


fn main() {
    bernoulli_example();
}