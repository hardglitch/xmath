// x.powf(2.0) + x : x1=-1, x2=0, x=-5..1 -> min=-0.25, max=20
// (x.powf(3.0)+1.0) * (x-1.0) / (x-2.0).sqrt() : No roots, x=-5..1 -> No extremums
// -1.0 / (x * (1.0 - (x.ln()).powf(2.0)).sqrt()) : No roots, x=-5..1 -> min=-25,24, max=-1
// x.powf(3.0) - 16.0*x.powf(2.0)/3.0 + 15.0*x : x1=0, x=-5..1 -> min=-333,33, max=10.66
// 6.0*x.powf(5.0) - 90.0*x.powf(3.0) - 5.0 : No roots, x=-5..1 -> min=-7505, max=967
// x.powf(3.0/2.0) - 3.0*x + 1.0 : No roots, x=1..9 -> min=-3, max=1
// 5.0 * x.powf(2.0) - 63.0*x - 18.0 :

use math::func_analysis::Expression;

fn main() {
    let mut y = Expression::new(
        |x: f64| x.powf(3.0) - 16.0*x.powf(2.0)/3.0 + 15.0*x
    );
    y.find_extremums(-5.0, 1.0);
    y.find_roots();
    y.print_result();

    // x1=0.00
    // Min F(x)=-333.33, x=-5.00
    // Max F(x)=10.66, x=1.00
}
