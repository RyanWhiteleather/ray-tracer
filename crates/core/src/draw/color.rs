use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use crate::math::approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.r, other.r) && approx_eq(self.g, other.g) && approx_eq(self.b, other.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::draw::Color;

    #[test]
    fn equals_returns_true_when_colors_are_equal() {
        let color1 = Color::new(4.3, -4.2, 3.1);
        let color2 = Color::new(4.3, -4.2, 3.1);

        assert_eq!(color1, color2);
        assert!(color1 == color2);
    }

    #[test]
    fn equals_returns_false_when_colors_are_not_equal() {
        let color1 = Color::new(4.3, -4.2, 3.1);
        let color2 = Color::new(10.0, -4.2, 3.1);

        assert_ne!(color1, color2);
        assert!(color1 != color2);
    }

    #[test]
    fn add_operator_returns_new_color_when_adding_two_colors() {
        let color1 = Color::new(3.0, -2.0, 5.0);
        let color2 = Color::new(-2.0, 3.0, 1.0);

        let result = color1 + color2;
        let expected = Color::new(1.0, 1.0, 6.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn subtract_operator_returns_new_color_when_subtracting_two_colors() {
        let color1 = Color::new(3.0, 2.0, 1.0);
        let color2 = Color::new(5.0, 6.0, 7.0);

        let result = color1 - color2;
        let expected = Color::new(-2.0, -4.0, -6.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn multiplication_operator_works_for_color_times_scalar() {
        let color = Color::new(1.0, -2.0, 3.0);
        let scale = 3.5;

        let result = color * scale;
        let expected = Color::new(3.5, -7.0, 10.5);

        assert_eq!(result, expected);
    }

    #[test]
    fn multiplication_operator_works_for_scalar_times_color() {
        let color = Color::new(1.0, -2.0, 3.0);
        let scale = 3.5;

        let result = scale * color;
        let expected = Color::new(3.5, -7.0, 10.5);

        assert_eq!(result, expected);
    }

    #[test]
    fn multiplication_operator_works_for_color_times_color() {
        let color1 = Color::new(1.0, 0.2, 0.4);
        let color2 = Color::new(0.9, 1.0, 0.1);

        let result = color1 * color2;
        let expected = Color::new(0.9, 0.2, 0.04);

        assert_eq!(result, expected);
    }

    #[test]
    fn division_operator_returns_scaled_color() {
        let color = Color::new(1.0, -2.0, 3.0);
        let scale = 2.0;

        let result = color / scale;
        let expected = Color::new(0.5, -1.0, 1.5);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(1.0, 2.0, 3.0, "(1.00, 2.00, 3.00)")]
    #[case(0.0, 0.0, 0.0, "(0.00, 0.00, 0.00)")]
    #[case(-1.234, 5.678, -9.1011, "(-1.23, 5.68, -9.10)")]
    #[case(0.0049, 0.005, 0.006, "(0.00, 0.01, 0.01)")]
    fn display_returns_expected_format(
        #[case] r: f64,
        #[case] g: f64,
        #[case] b: f64,
        #[case] expected: &str,
    ) {
        let color = Color::new(r, g, b);

        let result = color.to_string();

        assert_eq!(result, expected);
    }
}
