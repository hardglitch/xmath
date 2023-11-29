use std::error::Error;

pub fn quadratic_equation(a: f32, b: f32, c: f32) -> Result<Option<[f32; 2]>, Box<dyn Error>> {
    //! +-ax^2 +-bx +-с = 0

    if a == 0.0 { return Err("Argument 'a' must not be 0.".into()); }

    let d = b.powf(2.0) - 4.0 * a * c;
    if d < 0.0 { return Ok(None) }

    let x1 = (-b + d.sqrt()) / (2.0 * a);
    let x2 = (-b - d.sqrt()) / (2.0 * a);
    Ok(Some([x1, x2]))
}
