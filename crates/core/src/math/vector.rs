use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::{Point, approx_eq};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Compute the magnitude of a vector.
    /// This is the distance represented by that vector.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize a vector by converting it to a unit vecotr.
    /// This keeps all calculations anchored relative to a common scale.
    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    /// Compute the Dot product of two Vectors
    /// This returns the scalar value, used for intersecting rays and shading on a surface.
    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// Compute the cross product of two vectors.
    /// This returns a new vector that is perpendicular to both input vectors.
    pub fn cross(&self, v: &Vector) -> Vector {
        Vector::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn creates_a_vector() {
        let vector = Vector::new(4.0, -4.0, 3.0);
        assert_eq!(vector, Vector::new(4.0, -4.0, 3.0));
    }

    #[test]
    fn vectors_with_small_differences_are_equal() {
        let a = Vector::new(1.000001, 2.0, 3.0);
        let b = Vector::new(1.000002, 2.0, 3.0);

        assert_eq!(a, b);
    }

    #[test]
    fn vector_plus_point_returns_point() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);

        let result = v + p;

        assert_eq!(result, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn vector_plus_vector_returns_vector() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);

        let result = v1 + v2;

        assert_eq!(result, Vector::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn vector_minus_vector_returns_vector() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        let result = v1 - v2;

        assert_eq!(result, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn neg_vector_return_point() {
        let v = Vector::new(1.0, -2.0, 3.0);

        let result = -v;

        assert_eq!(result, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn vector_can_be_multiplied_by_a_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);

        let result = v * 3.5;

        assert_eq!(result, Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn scalar_can_be_multiplied_by_a_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);

        let result = 3.5 * v;

        assert_eq!(result, Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn vector_scalar_multiplication_is_commutative_for_same_scalar() {
        let v = Vector::new(2.0, 4.0, -6.0);

        assert_eq!(v * 2.0, 2.0 * v);
    }

    #[test]
    fn vector_can_be_divided_by_a_scalar() {
        let v = Vector::new(6.0, -4.0, 2.0);

        let result = v / 2.0;

        assert_eq!(result, Vector::new(3.0, -2.0, 1.0));
    }

    #[rstest]
    #[case(Vector::new(1.0, 0.0, 0.0), 1.0)]
    #[case(Vector::new(0.0, 1.0, 0.0), 1.0)]
    #[case(Vector::new(0.0, 0.0, 1.0), 1.0)]
    #[case(Vector::new(1.0, 2.0, 3.0), (14.0_f64).sqrt())]
    #[case(Vector::new(-1.0, -2.0, -3.0), (14.0_f64).sqrt())]
    fn magnitude_of_vectors_returns_expected_values(#[case] vector: Vector, #[case] expected: f64) {
        assert_eq!(vector.magnitude(), expected);
    }

    #[rstest]
    #[case(Vector::new(4.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0))]
    #[case(Vector::new(1.0, 2.0, 3.0), Vector::new(
    1.0 / (14.0_f64).sqrt(),
    2.0 / (14.0_f64).sqrt(),
    3.0 / (14.0_f64).sqrt()
))]
    fn normalize_vector_returns_expected_vector(#[case] vector: Vector, #[case] expected: Vector) {
        let result = vector.normalize();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(Vector::new(4.0, 0.0, 0.0), 1.0)]
    #[case(Vector::new(1.0, 2.0, 3.0), 1.0)]
    fn magnitude_of_normalized_vector_is_one(#[case] vector: Vector, #[case] expected: f64) {
        let result = vector.normalize().magnitude();

        assert_eq!(result, expected);
    }

    #[test]
    fn dot_product_of_two_vectors_returns_scalar() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        let result = v1.dot(&v2);

        assert_eq!(result, 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(&v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Vector::new(1.0, -2.0, 1.0));
    }
}
