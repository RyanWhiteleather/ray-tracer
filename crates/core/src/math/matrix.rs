use std::fmt;
use std::ops::{Index, IndexMut, Mul};

use crate::math::{point::Point, vector::Vector};

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    elements: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            elements: vec![0.0; rows * cols],
        }
    }

    pub fn from_elements(rows: usize, cols: usize, elements: Vec<f64>) -> Self {
        assert_eq!(
            rows * cols,
            elements.len(),
            "matrix dimensions do not match element count"
        );

        Self {
            rows,
            cols,
            elements,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn elements(&self) -> &[f64] {
        &self.elements
    }

    pub fn transpose(&self) -> Matrix {
        let mut transposed = Matrix::new(self.cols, self.rows);

        for col in 0..self.cols {
            for row in 0..self.rows {
                transposed[(col, row)] = self[(row, col)];
            }
        }

        transposed
    }

    pub fn determinant(&self) -> f64 {
        assert_eq!(self.rows, self.cols, "determinant requires a square matrix");

        if self.rows == 2 && self.cols == 2 {
            return self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)];
        }

        let mut determinant = 0.0;
        for col in 0..self.cols {
            determinant += self[(0, col)] * self.cofactor(0, col);
        }

        determinant
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix {
        if !self.is_invertible() {
            panic!("Matrix is not invertible");
        }

        let mut inverse = Matrix::new(self.rows, self.cols);
        let determinant = self.determinant();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let cofactor = self.cofactor(row, col);

                // Transpose while assigning.
                inverse[(col, row)] = cofactor / determinant;
            }
        }

        inverse
    }

    pub fn submatrix(&self, row_to_remove: usize, col_to_remove: usize) -> Matrix {
        let mut result = Matrix::new(self.rows - 1, self.cols - 1);

        let mut result_row = 0;
        for row in 0..self.rows {
            if row == row_to_remove {
                continue;
            }

            let mut result_col = 0;
            for col in 0..self.cols {
                if col == col_to_remove {
                    continue;
                }

                result[(result_row, result_col)] = self[(row, col)];
                result_col += 1;
            }

            result_row += 1;
        }

        result
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 != 0 { -minor } else { minor }
    }

    pub fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
        assert_eq!(
            a.cols, b.rows,
            "matrix multiplication requires left cols == right rows"
        );

        let mut result = Matrix::new(a.rows, b.cols);

        for row in 0..a.rows {
            for col in 0..b.cols {
                let mut sum = 0.0;

                for shared in 0..a.cols {
                    sum += a[(row, shared)] * b[(shared, col)];
                }

                result[(row, col)] = sum;
            }
        }

        result
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let mut result = Matrix::identity_4x4();
        result[(0, 3)] = x;
        result[(1, 3)] = y;
        result[(2, 3)] = z;
        result
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let mut result = Matrix::identity_4x4();
        result[(0, 0)] = x;
        result[(1, 1)] = y;
        result[(2, 2)] = z;
        result
    }

    pub fn rotation_x(radians: f64) -> Matrix {
        let mut result = Matrix::identity_4x4();

        result[(1, 1)] = radians.cos();
        result[(1, 2)] = -radians.sin();
        result[(2, 1)] = radians.sin();
        result[(2, 2)] = radians.cos();

        result
    }

    pub fn rotation_y(radians: f64) -> Matrix {
        let mut result = Matrix::identity_4x4();

        result[(0, 0)] = radians.cos();
        result[(0, 2)] = radians.sin();
        result[(2, 0)] = -radians.sin();
        result[(2, 2)] = radians.cos();

        result
    }

    pub fn rotation_z(radians: f64) -> Matrix {
        let mut result = Matrix::identity_4x4();

        result[(0, 0)] = radians.cos();
        result[(0, 1)] = -radians.sin();
        result[(1, 0)] = radians.sin();
        result[(1, 1)] = radians.cos();

        result
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        let mut result = Matrix::identity_4x4();

        result[(0, 1)] = xy;
        result[(0, 2)] = xz;
        result[(1, 0)] = yx;
        result[(1, 2)] = yz;
        result[(2, 0)] = zx;
        result[(2, 1)] = zy;

        result
    }

    pub fn identity(size: usize) -> Matrix {
        let mut m = Matrix::new(size, size);

        for i in 0..size {
            m[(i, i)] = 1.0;
        }

        m
    }

    pub fn identity_4x4() -> Matrix {
        Matrix::identity(4)
    }

    fn index_for(&self, row: usize, col: usize) -> usize {
        assert!(row < self.rows, "row out of bounds: {row}");
        assert!(col < self.cols, "col out of bounds: {col}");
        row * self.cols + col
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.elements == other.elements
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        let i = self.index_for(row, col);
        &self.elements[i]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        let i = self.index_for(row, col);
        &mut self.elements[i]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix ({}x{})", self.rows, self.cols)?;

        for row in 0..self.rows {
            write!(f, "[ ")?;
            for col in 0..self.cols {
                write!(f, "{:.5}", self[(row, col)])?;
                if col < self.cols - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, " ]")?;
        }

        Ok(())
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        Matrix::multiply(self, rhs)
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        Matrix::multiply(&self, rhs)
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix::multiply(self, &rhs)
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix::multiply(&self, &rhs)
    }
}

impl Mul<Point> for &Matrix {
    type Output = Point;

    fn mul(self, p: Point) -> Self::Output {
        let x = self[(0, 0)] * p.x + self[(0, 1)] * p.y + self[(0, 2)] * p.z + self[(0, 3)];
        let y = self[(1, 0)] * p.x + self[(1, 1)] * p.y + self[(1, 2)] * p.z + self[(1, 3)];
        let z = self[(2, 0)] * p.x + self[(2, 1)] * p.y + self[(2, 2)] * p.z + self[(2, 3)];
        Point::new(x, y, z)
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, p: Point) -> Self::Output {
        (&self).mul(p)
    }
}

impl Mul<Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, v: Vector) -> Self::Output {
        let x = self[(0, 0)] * v.x + self[(0, 1)] * v.y + self[(0, 2)] * v.z;
        let y = self[(1, 0)] * v.x + self[(1, 1)] * v.y + self[(1, 2)] * v.z;
        let z = self[(2, 0)] * v.x + self[(2, 1)] * v.y + self[(2, 2)] * v.z;
        Vector::new(x, y, z)
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, v: Vector) -> Self::Output {
        (&self).mul(v)
    }
}

impl Mul<&Matrix> for Point {
    type Output = Point;

    fn mul(self, m: &Matrix) -> Self::Output {
        m * self
    }
}

impl Mul<Matrix> for Point {
    type Output = Point;

    fn mul(self, m: Matrix) -> Self::Output {
        &m * self
    }
}

impl Mul<&Matrix> for Vector {
    type Output = Vector;

    fn mul(self, m: &Matrix) -> Self::Output {
        m * self
    }
}

impl Mul<Matrix> for Vector {
    type Output = Vector;

    fn mul(self, m: Matrix) -> Self::Output {
        &m * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{point::Point, vector::Vector};

    const EPSILON: f64 = 0.00001;
    const HALF_QUARTER: f64 = std::f64::consts::PI / 4.0;
    const FULL_QUARTER: f64 = std::f64::consts::PI / 2.0;

    fn assert_float_eq(a: f64, b: f64) {
        assert!((a - b).abs() < EPSILON, "expected {b}, got {a}");
    }

    fn assert_matrix_approx_eq(actual: &Matrix, expected: &Matrix) {
        assert_eq!(actual.rows(), expected.rows());
        assert_eq!(actual.cols(), expected.cols());

        for row in 0..actual.rows() {
            for col in 0..actual.cols() {
                assert_float_eq(actual[(row, col)], expected[(row, col)]);
            }
        }
    }

    #[test]
    fn new_matrix_creates_and_sets_4x4() {
        let matrix = Matrix::from_elements(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ],
        );

        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(0, 3)], 4.0);
        assert_eq!(matrix[(1, 0)], 5.5);
        assert_eq!(matrix[(1, 2)], 7.5);
        assert_eq!(matrix[(2, 2)], 11.0);
        assert_eq!(matrix[(3, 0)], 13.5);
        assert_eq!(matrix[(3, 2)], 15.5);
    }

    #[test]
    fn new_matrix_creates_and_sets_2x2() {
        let matrix = Matrix::from_elements(2, 2, vec![-3.0, 5.0, 1.0, -2.0]);

        assert_eq!(matrix[(0, 0)], -3.0);
        assert_eq!(matrix[(0, 1)], 5.0);
        assert_eq!(matrix[(1, 0)], 1.0);
        assert_eq!(matrix[(1, 1)], -2.0);
    }

    #[test]
    fn new_matrix_creates_and_sets_3x3() {
        let matrix =
            Matrix::from_elements(3, 3, vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(matrix[(0, 0)], -3.0);
        assert_eq!(matrix[(1, 1)], -2.0);
        assert_eq!(matrix[(2, 2)], 1.0);
    }

    #[test]
    fn equals_returns_true_when_matrices_are_equal() {
        let a = Matrix::from_elements(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ],
        );

        let b = Matrix::from_elements(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ],
        );

        assert_eq!(a, b);
    }

    #[test]
    fn equals_returns_false_when_matrices_are_not_equal() {
        let a = Matrix::from_elements(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ],
        );

        let b = Matrix::from_elements(
            4,
            4,
            vec![
                0.0, 0.0, 0.0, 0.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ],
        );

        assert_ne!(a, b);

        let b = Matrix::from_elements(
            4,
            5,
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 6.0, 7.0, 8.0, 0.0, 9.0, 10.0, 11.0, 12.0, 0.0, 13.0,
                14.0, 15.0, 16.0, 0.0,
            ],
        );

        assert_ne!(a, b);

        let b = Matrix::from_elements(
            3,
            4,
            vec![
                0.0, 0.0, 0.0, 0.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        );

        assert_ne!(a, b);
    }

    #[test]
    #[should_panic]
    fn getting_with_row_beyond_max_throws() {
        let m = Matrix::new(3, 3);
        let _ = m[(3, 0)];
    }

    #[test]
    #[should_panic]
    fn getting_with_column_beyond_max_throws() {
        let m = Matrix::new(3, 3);
        let _ = m[(0, 3)];
    }

    #[test]
    #[should_panic]
    fn setting_with_row_beyond_max_throws() {
        let mut m = Matrix::new(3, 3);
        m[(3, 0)] = 5.0;
    }

    #[test]
    #[should_panic]
    fn setting_with_column_beyond_max_throws() {
        let mut m = Matrix::new(3, 3);
        m[(0, 3)] = 5.0;
    }

    #[test]
    fn multiply_returns_new_matrix() {
        let a = Matrix::from_elements(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        let b = Matrix::from_elements(
            4,
            4,
            vec![
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ],
        );

        let expected = Matrix::from_elements(
            4,
            4,
            vec![
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ],
        );

        let result = a * b;
        assert_eq!(result, expected);
    }

    #[test]
    fn multiply_matrix_by_point_returns_new_point() {
        let a = Matrix::from_elements(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );

        let b = Point::new(1.0, 2.0, 3.0);
        let expected = Point::new(18.0, 24.0, 33.0);
        let result = a * b;

        assert_eq!(result, expected);
    }

    #[test]
    fn multiply_matrix_by_vector_returns_new_vector() {
        let a = Matrix::from_elements(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );

        let b = Vector::new(1.0, 2.0, 3.0);
        let expected = Vector::new(14.0, 22.0, 32.0);
        let result = a * b;

        assert_eq!(result, expected);
    }

    #[test]
    fn identity_returns_new_identity_matrix() {
        let actual = Matrix::identity(2);
        let expected = Matrix::from_elements(2, 2, vec![1.0, 0.0, 0.0, 1.0]);
        assert_eq!(actual, expected);

        let actual = Matrix::identity(3);
        let expected =
            Matrix::from_elements(3, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        assert_eq!(actual, expected);

        let actual = Matrix::identity_4x4();
        let expected = Matrix::from_elements(
            4,
            4,
            vec![
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_matrix_by_identity_returns_same_matrix() {
        let a = Matrix::from_elements(
            4,
            4,
            vec![
                0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
            ],
        );

        let result = a.clone() * Matrix::identity_4x4();
        assert_eq!(result, a);
    }

    #[test]
    fn transpose_returns_new_matrix() {
        let a = Matrix::from_elements(
            4,
            4,
            vec![
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ],
        );

        let result = a.transpose();

        let expected = Matrix::from_elements(
            4,
            4,
            vec![
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
            ],
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn transpose_3x4_returns_new_matrix() {
        let a = Matrix::from_elements(
            4,
            3,
            vec![0.0, 9.0, 3.0, 9.0, 8.0, 0.0, 1.0, 8.0, 5.0, 0.0, 0.0, 5.0],
        );

        let result = a.transpose();

        let expected = Matrix::from_elements(
            3,
            4,
            vec![0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0],
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn transpose_identity_matrix_returns_identity_matrix() {
        let result = Matrix::identity_4x4().transpose();
        let expected = Matrix::identity_4x4();
        assert_eq!(result, expected);
    }

    #[test]
    fn determinant_returns_correct_value() {
        let m = Matrix::from_elements(2, 2, vec![1.0, 5.0, -3.0, 2.0]);

        let result = m.determinant();
        assert_eq!(result, 17.0);
    }

    #[test]
    fn submatrix_returns_new_matrix_with_removed_row_and_column() {
        let m = Matrix::from_elements(3, 3, vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);

        let result = m.submatrix(0, 2);

        let expected = Matrix::from_elements(2, 2, vec![-3.0, 2.0, 0.0, 6.0]);

        assert_eq!(result, expected);

        let m = Matrix::from_elements(
            4,
            4,
            vec![
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ],
        );

        let result = m.submatrix(2, 1);

        let expected =
            Matrix::from_elements(3, 3, vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]);

        assert_eq!(result, expected);
    }

    #[test]
    fn minor_returns_correct_value() {
        let m = Matrix::from_elements(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        let result = m.minor(1, 0);
        assert_eq!(result, 25.0);
    }

    #[test]
    fn cofactor_returns_correct_value() {
        let m = Matrix::from_elements(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_3x3_returns_correct_value() {
        let m = Matrix::from_elements(3, 3, vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);

        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn determinant_4x4_returns_correct_value() {
        let m = Matrix::from_elements(
            4,
            4,
            vec![
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            ],
        );

        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn is_invertible_returns_correct_value() {
        let m = Matrix::from_elements(
            4,
            4,
            vec![
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ],
        );

        assert!(m.is_invertible());

        let m = Matrix::from_elements(
            4,
            4,
            vec![
                -4.0, 2.0, -2.0, 3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ],
        );

        assert!(!m.is_invertible());
    }

    #[test]
    fn inverse_returns_correct_value() {
        let m = Matrix::from_elements(
            4,
            4,
            vec![
                -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0,
                4.0,
            ],
        );

        let result = m.inverse();

        let expected = Matrix::from_elements(
            4,
            4,
            vec![
                0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
            ],
        );

        assert_eq!(m.determinant(), 532.0);
        assert_eq!(m.cofactor(2, 3), -160.0);
        assert_float_eq(result[(3, 2)], -160.0 / 532.0);
        assert_eq!(m.cofactor(3, 2), 105.0);
        assert_float_eq(result[(2, 3)], 105.0 / 532.0);
        assert_matrix_approx_eq(&result, &expected);

        let m = Matrix::from_elements(
            4,
            4,
            vec![
                8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
            ],
        );

        let result = m.inverse();
        let expected = Matrix::from_elements(
            4,
            4,
            vec![
                -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077,
                0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
            ],
        );
        assert_matrix_approx_eq(&result, &expected);

        let m = Matrix::from_elements(
            4,
            4,
            vec![
                9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0,
                2.0,
            ],
        );

        let result = m.inverse();
        let expected = Matrix::from_elements(
            4,
            4,
            vec![
                -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333,
                -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
            ],
        );
        assert_matrix_approx_eq(&result, &expected);
    }

    #[test]
    #[should_panic]
    fn inverse_throws_when_not_invertible() {
        let m = Matrix::from_elements(
            4,
            4,
            vec![
                -4.0, 2.0, -2.0, 3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ],
        );

        let _ = m.inverse();
    }

    #[test]
    fn multiply_by_inverse_returns_original_matrix() {
        let a = Matrix::from_elements(
            4,
            4,
            vec![
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            ],
        );

        let b = Matrix::from_elements(
            4,
            4,
            vec![
                8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
            ],
        );

        let c = a.clone() * b.clone();
        let result = c * b.inverse();

        assert_matrix_approx_eq(&result, &a);
    }

    #[test]
    fn translation_returns_correct_matrix() {
        let result = Matrix::translation(5.0, -3.0, 2.0);
        let expected = Matrix::from_elements(
            4,
            4,
            vec![
                1.0, 0.0, 0.0, 5.0, 0.0, 1.0, 0.0, -3.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );

        assert_matrix_approx_eq(&result, &expected);
    }

    #[test]
    fn translation_multiply_by_point_returns_correct_point() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        let result = transform * p;
        let expected = Point::new(2.0, 1.0, 7.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn translation_multiply_by_inverse_returns_correct_point() {
        let inverse = Matrix::translation(5.0, -3.0, 2.0).inverse();
        let p = Point::new(-3.0, 4.0, 5.0);
        let result = inverse * p;
        let expected = Point::new(-8.0, 7.0, 3.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn translation_multiply_by_vector_returns_same_vector() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        let result = transform * v;

        assert_eq!(result, v);
    }

    #[test]
    fn scaling_multiply_by_point_returns_correct_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        let result = transform * p;
        let expected = Point::new(-8.0, 18.0, 32.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn scaling_multiply_by_inverse_returns_correct_point() {
        let inverse = Matrix::scaling(2.0, 3.0, 4.0).inverse();
        let p = Point::new(-4.0, 6.0, 8.0);
        let result = inverse * p;
        let expected = Point::new(-2.0, 2.0, 2.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn scaling_multiply_by_vector_returns_correct_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        let result = transform * v;
        let expected = Vector::new(-8.0, 18.0, 32.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn scaling_multiply_by_inverse_returns_correct_vector() {
        let inverse = Matrix::scaling(2.0, 3.0, 4.0).inverse();
        let v = Vector::new(-4.0, 6.0, 8.0);
        let result = inverse * v;
        let expected = Vector::new(-2.0, 2.0, 2.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn scaling_reflects_a_point() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let result = transform * p;
        let expected = Point::new(-2.0, 3.0, 4.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn rotation_x_rotates_point() {
        let p = Point::new(0.0, 1.0, 0.0);

        let rotation = Matrix::rotation_x(HALF_QUARTER);
        let result = rotation * p;
        let expected = Point::new(0.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0);
        assert_eq!(result, expected);

        let rotation = Matrix::rotation_x(FULL_QUARTER);
        let result = rotation * p;
        let expected = Point::new(0.0, 0.0, 1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn rotation_x_inverse_rotates_point() {
        let p = Point::new(0.0, 1.0, 0.0);

        let rotation = Matrix::rotation_x(HALF_QUARTER).inverse();
        let result = rotation * p;
        let expected = Point::new(0.0, (2.0_f64).sqrt() / 2.0, -((2.0_f64).sqrt() / 2.0));

        assert_eq!(result, expected);
    }

    #[test]
    fn rotation_y_rotates_point() {
        let p = Point::new(0.0, 0.0, 1.0);

        let rotation = Matrix::rotation_y(HALF_QUARTER);
        let result = rotation * p;
        let expected = Point::new((2.0_f64).sqrt() / 2.0, 0.0, (2.0_f64).sqrt() / 2.0);
        assert_eq!(result, expected);

        let rotation = Matrix::rotation_y(FULL_QUARTER);
        let result = rotation * p;
        let expected = Point::new(1.0, 0.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn rotation_z_rotates_point() {
        let p = Point::new(0.0, 1.0, 0.0);

        let rotation = Matrix::rotation_z(HALF_QUARTER);
        let result = rotation * p;
        let expected = Point::new(-((2.0_f64).sqrt() / 2.0), (2.0_f64).sqrt() / 2.0, 0.0);
        assert_eq!(result, expected);

        let rotation = Matrix::rotation_z(FULL_QUARTER);
        let result = rotation * p;
        let expected = Point::new(-1.0, 0.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn shearing_transforms_point_x_in_proportion_to_y() {
        let point = Point::new(2.0, 3.0, 4.0);
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let expected = Point::new(5.0, 3.0, 4.0);

        assert_eq!(transform * point, expected);
    }

    #[test]
    fn shearing_transforms_point_x_in_proportion_to_z() {
        let point = Point::new(2.0, 3.0, 4.0);
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let expected = Point::new(6.0, 3.0, 4.0);

        assert_eq!(transform * point, expected);
    }

    #[test]
    fn shearing_transforms_point_y_in_proportion_to_x() {
        let point = Point::new(2.0, 3.0, 4.0);
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let expected = Point::new(2.0, 5.0, 4.0);

        assert_eq!(transform * point, expected);
    }

    #[test]
    fn shearing_transforms_point_y_in_proportion_to_z() {
        let point = Point::new(2.0, 3.0, 4.0);
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let expected = Point::new(2.0, 7.0, 4.0);

        assert_eq!(transform * point, expected);
    }

    #[test]
    fn shearing_transforms_point_z_in_proportion_to_x() {
        let point = Point::new(2.0, 3.0, 4.0);
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let expected = Point::new(2.0, 3.0, 6.0);

        assert_eq!(transform * point, expected);
    }

    #[test]
    fn shearing_transforms_point_z_in_proportion_to_y() {
        let point = Point::new(2.0, 3.0, 4.0);
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let expected = Point::new(2.0, 3.0, 7.0);

        assert_eq!(transform * point, expected);
    }

    #[test]
    fn transformation_applied_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(FULL_QUARTER);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = a.clone() * p;
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));

        let p3 = b.clone() * p2;
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0));

        let p4 = c.clone() * p3;
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));

        let t = c * b * a;
        let result = t * p;

        assert_eq!(result, Point::new(15.0, 0.0, 7.0));
    }
}
