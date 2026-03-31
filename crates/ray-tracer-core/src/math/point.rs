use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::{Vector, approx_eq};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_point() {
        let point = Point::new(4.0, -4.0, 3.0);
        assert_eq!(point, Point::new(4.0, -4.0, 3.0));
    }

    #[test]
    fn points_with_small_differences_are_equal() {
        let a = Point::new(1.000001, 2.0, 3.0);
        let b = Point::new(1.000002, 2.0, 3.0);

        assert_eq!(a, b);
    }

    #[test]
    fn point_plus_vector_returns_point() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);

        let result = p + v;

        assert_eq!(result, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn point_minus_point_returns_vector() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        let result = p1 - p2;

        assert_eq!(result, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn point_minus_vector_returns_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        let result = p - v;

        assert_eq!(result, Point::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn neg_point_return_point() {
        let p = Point::new(1.0, -2.0, 3.0);

        let result = -p;

        assert_eq!(result, Point::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn point_can_be_multiplied_by_a_scalar() {
        let p = Point::new(1.0, -2.0, 3.0);

        let result = p * 3.5;

        assert_eq!(result, Point::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn scalar_can_be_multiplied_by_a_point() {
        let p = Point::new(1.0, -2.0, 3.0);

        let result = 3.5 * p;

        assert_eq!(result, Point::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn point_scalar_multiplication_is_commutative_for_same_scalar() {
        let p = Point::new(2.0, 4.0, -6.0);

        assert_eq!(p * 2.0, 2.0 * p);
    }

    #[test]
    fn point_can_be_divided_by_a_scalar() {
        let p = Point::new(6.0, -4.0, 2.0);

        let result = p / 2.0;

        assert_eq!(result, Point::new(3.0, -2.0, 1.0));
    }
}
