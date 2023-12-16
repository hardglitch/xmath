use xmath::im_numbers::ImExpr;

fn main() {
    let b = ImExpr::new("2 + 5i + 5 + 4.7i");
    b.execute();
}