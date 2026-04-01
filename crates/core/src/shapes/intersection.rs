use crate::shapes::Shape;

pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a dyn Shape) -> Self {
        Self { t, object }
    }
}