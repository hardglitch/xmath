use math::parser::Expression;

fn main() {

    let mut expr = Expression::new("x+1+2");
    expr.print_result();
}
