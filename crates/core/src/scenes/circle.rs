use crate::{draw::{Canvas, Color}, math::Point, rays::Ray, shapes::Sphere};

pub fn render_circle() -> Canvas {
    let canvas_pixels = 500usize;
    let wall_size = 7.0;
    let wall_z = 10.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);

    let shape = Sphere::new();
    // Transformation examples:
    // let mut shape = Sphere::new();
    // shape.set_transform(Matrix::scaling(1.0, 0.5, 1.0));
    // shape.set_transform(Matrix::scaling(0.5, 1.0, 1.0));
    // shape.set_transform(Matrix::identity_4x4().rotate_z(HALF_QUARTER).scale(0.5, 1.0, 1.0));
    // shape.set_transform(Matrix::identity_4x4().shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).scale(0.5, 1.0, 1.0));

    let half = wall_size / 2.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let ray_origin = Point::new(0.0, 0.0, -5.0);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let target = Point::new(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (target - ray_origin).normalize());
            let intersections = ray.intersect(&shape);

            if intersections.hit().is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    canvas
}