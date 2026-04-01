use std::ops::{Index, IndexMut};

use crate::draw::Color;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = self.index_for(x, y);
        self.pixels[index] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.pixels[y * self.width + x])
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();

        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");

        for y in 0..self.height {
            let mut components: Vec<String> = Vec::with_capacity(self.width * 3);

            for x in 0..self.width {
                let c = self[(x, y)];

                components.push(scale_to_255(c.r).to_string());
                components.push(scale_to_255(c.g).to_string());
                components.push(scale_to_255(c.b).to_string());
            }

            let row = components.join(" ");

            for line in wrap_at_70(&row) {
                ppm.push_str(&line);
                ppm.push('\n');
            }
        }

        ppm
    }

    fn index_for(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width, "x out of bounds: {x}");
        assert!(y < self.height, "y out of bounds: {y}");
        y * self.width + x
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        let i = self.index_for(x, y);
        &self.pixels[i]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        let i = self.index_for(x, y);
        &mut self.pixels[i]
    }
}

fn wrap_at_70(text: &str) -> Vec<String> {
    const MAX: usize = 70;

    let mut lines = Vec::new();
    let mut remaining = text;

    while remaining.len() > MAX {
        let break_pos = remaining[..=MAX].rfind(' ').unwrap_or(MAX);

        lines.push(remaining[..break_pos].to_string());
        remaining = &remaining[break_pos + 1..];
    }

    lines.push(remaining.to_string());
    lines
}

fn scale_to_255(value: f64) -> i32 {
    let clamped = value.clamp(0.0, 1.0);
    (clamped * 255.0).round() as i32
}

#[cfg(test)]
mod tests {
    use crate::draw::Canvas;
    use crate::draw::Color;

    #[test]
    fn new_canvas_returns_canvas_with_correct_size_and_empty_colors() {
        let width = 10;
        let height = 20;
        let canvas = Canvas::new(width, height);

        assert_eq!(canvas.pixels.len(), width * height);

        let empty_color = Color::new(0.0, 0.0, 0.0);
        for pixel in &canvas.pixels {
            assert_eq!(*pixel, empty_color);
        }
    }

    #[test]
    fn write_pixel_sets_pixel_at_correct_coordinates() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        canvas.write_pixel(2, 3, red);

        assert_eq!(canvas.pixel_at(2, 3), Some(red));
    }

    #[test]
    fn write_pixel_ignores_write_when_outside_canvas_boundary() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        let empty = Color::new(0.0, 0.0, 0.0);

        canvas.write_pixel(11, 21, red);
        canvas.write_pixel(9, 21, red);
        canvas.write_pixel(10, 10, red);
        canvas.write_pixel(0, 20, red);

        assert_eq!(canvas.pixel_at(9, 19), Some(empty));
        assert_eq!(canvas.pixel_at(0, 0), Some(empty));
    }

    #[test]
    fn index_mut_sets_pixel_at_correct_coordinates() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        canvas[(2, 3)] = red;

        assert_eq!(canvas.pixel_at(2, 3), Some(red));
    }

    #[test]
    fn index_returns_pixel_at_correct_coordinates() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        canvas.write_pixel(2, 3, red);

        assert_eq!(canvas[(2, 3)], red);
    }

    #[test]
    fn pixel_at_returns_none_when_outside_canvas_boundary() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(canvas.pixel_at(11, 10), None);
        assert_eq!(canvas.pixel_at(9, 21), None);
        assert_eq!(canvas.pixel_at(10, 10), None);
        assert_eq!(canvas.pixel_at(9, 20), None);
    }

    #[test]
    fn to_ppm_builds_correct_ppm_header() {
        let canvas = Canvas::new(5, 3);

        let result = canvas.to_ppm();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn to_ppm_builds_correct_pixel_string() {
        let mut canvas = Canvas::new(5, 3);

        let color1 = Color::new(1.5, 0.0, 0.0);
        let color2 = Color::new(0.0, 0.5, 0.0);
        let color3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, color1);
        canvas.write_pixel(2, 1, color2);
        canvas.write_pixel(4, 2, color3);

        let result = canvas.to_ppm();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn to_ppm_builds_correct_pixel_string_with_max_character_limit() {
        let mut canvas = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);

        for y in 0..canvas.height {
            for x in 0..canvas.width {
                canvas.write_pixel(x, y, color);
            }
        }

        let ppm = canvas.to_ppm();
        let lines: Vec<&str> = ppm.lines().collect();

        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn to_ppm_ends_with_newline() {
        let canvas = Canvas::new(10, 2);
        let ppm = canvas.to_ppm();

        assert!(ppm.ends_with('\n'));
    }
}
