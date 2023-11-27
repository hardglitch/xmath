#[allow(dead_code)]
fn set_func(x: f64) -> f64 {
    // -- write your equation here --

    // x.powf(2.0) + x    //  roots = [-1.0, 0.0], x=-5..1 -> min=-0.25, max=20
    // (x.powf(3.0) + 1.0) * (x - 1.0) / (x - 2.0).sqrt()   //  [] - no roots, x=-5..1 -> min=0, max=0
    // -1.0 / (x * (1.0 - (x.ln()).powf(2.0)).sqrt())  //  [] - no roots, x=-5..1 -> min=-25,24, max=0
    // x.powf(3.0) - 16.0 * x.powf(2.0) / 3.0 + 15.0 * x  //  roots = [0], x=-5..1 -> min=-333,33, max=10.59
    // 6.0 * x.powf(5.0) - 90.0 * x.powf(3.0) - 5.0   //  x=-5..1 -> min=-7505, max=967
    x.powf(3.0/2.0) - 3.0 * x + 1.0  // x=1..9 -> min=-3, max=0.98
}

#[allow(dead_code)]
fn get_roots() {
    let mut roots = Vec::<f64>::new();
    for i in i16::MIN..i16::MAX {
        let x = i as f64 / 100.0;
        let y = set_func(x);
        if (0.0 - y).abs() <= 0.001 {
            roots.push(x)
        }
    }
    println!("{:?}", roots);
}

#[allow(dead_code)]
fn get_func_extremums(x_min: i16, x_max: i16) {
    if x_min > x_max { panic!("x_min > x_max") }
    let mut extrs = Vec::<i32>::new();
    let scale = 100;

    for i in x_min * scale..x_max * scale {
        let x = i as f64 / scale as f64;
        let y = set_func(x);
        extrs.push((y * scale as f64) as i32)
    }

    extrs.sort();
    let min = *extrs.first().unwrap() as f64 / scale as f64;
    let max = *extrs.last().unwrap() as f64 / scale as f64;
    println!("Min = {:?}", min);
    println!("Max = {:?}", max);
}

fn main() {
    // get_roots();
    get_func_extremums(1, 9);
}
