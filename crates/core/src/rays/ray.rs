use crate::{math::{Matrix, Point, Vector}, shapes::{Intersection, Intersections, Shape}};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    /// Compute the point at the given distance (time).
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    /// Compute the intersections of a ray with a shape.
    /// TODO - This is specific to sphere now, needs to be changed to be on the shpae in stead of the ray.
    pub fn intersect<'a>(&self, shape: &'a dyn Shape) -> Intersections<'a> {
        let ray = self.transform(&shape.transform().inverse());

        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::new(vec![]);
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        Intersections::new(vec![
            Intersection::new(t1, shape),
            Intersection::new(t2, shape),
        ])
    }

    pub fn transform(&self, transformation: &Matrix) -> Self {
        Self {
            origin: transformation * self.origin,
            direction: transformation * self.direction,
        }
    }
}