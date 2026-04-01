use crate::shapes::Intersection;

pub struct Intersections<'a> {
    items: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new(mut items: Vec<Intersection<'a>>) -> Self {
        items.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Self { items }
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<&Intersection<'a>> {
        self.items.get(index)
    }

    /// Returns the visible intersection from the ray's origin.
    /// This is the lowest non-negative value.
    pub fn hit(&self) -> Option<&Intersection<'a>> {
        self.items.iter().find(|x| x.t >= 0.0)
    }
}