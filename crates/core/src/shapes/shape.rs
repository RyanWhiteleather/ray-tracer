use crate::math::Matrix;

pub trait Shape {
    fn transform(&self) -> &Matrix;
    fn set_transform(&mut self, transform: Matrix);
}