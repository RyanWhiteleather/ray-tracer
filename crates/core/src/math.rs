pub mod matrix;
pub mod point;
pub mod vector;

pub use matrix::Matrix;
pub use point::Point;
pub use vector::Vector;

pub const EPSILON: f64 = 1e-5;

pub fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
