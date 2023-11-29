use std::error::Error;

#[derive(PartialEq, Debug)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[allow(dead_code)]
pub fn cos_alpha(a: &Vector3D, b: &Vector3D) -> Result<f32, Box<dyn Error>> {
    let ab = a.x * b.x + a.y * b.y + a.z * b.z;
    let a = (a.x.powf(2.0) + a.y.powf(2.0) + a.z.powf(2.0)).sqrt();
    let b = (b.x.powf(2.0) + b.y.powf(2.0) + b.z.powf(2.0)).sqrt();
    Ok(ab / (a * b))
}

#[allow(dead_code)]
pub fn vector_multiplication(a: &Vector3D, b: &Vector3D) -> Vector3D {
    Vector3D{
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}
