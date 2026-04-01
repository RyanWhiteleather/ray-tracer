use std::f64::consts::PI;

use crate::{
    canvas::Canvas,
    color::Color,
    math::{Matrix, Point},
};

pub fn render_clock() -> Canvas {
    let mut canvas = Canvas::new(250, 250);
    let white = Color::new(1.0, 1.0, 1.0);

    let rotation = Matrix::rotation_y(PI * 2.0 / 12.0);

    let transform = Matrix::identity_4x4()
        * Matrix::translation(125.0, 0.0, 125.0)
        * Matrix::scaling(100.0, 100.0, 100.0);

    let mut point = Point::new(0.0, 0.0, 1.0);

    for _ in 0..12 {
        let p = &transform * point;

        let x = p.x.round() as usize;
        let y = (250.0 - p.z).round() as usize;

        canvas.write_pixel(x, y, white);

        point = &rotation * point;
    }

    canvas
}
