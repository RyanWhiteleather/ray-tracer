use crate::{math::Matrix, shapes::Shape};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub transform: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transform: Matrix::identity_4x4(),
        }
    }
}

impl Shape for Sphere {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}