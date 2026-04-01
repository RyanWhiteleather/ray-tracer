use crate::{
    draw::Canvas,
    draw::Color,
    math::{Point, Vector},
};

#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

#[derive(Debug, Clone, Copy)]
pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

pub fn render_projectile() -> Canvas {
    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let mut canvas = Canvas::new(900, 550);
    let red = Color::new(1.0, 0.0, 0.0);

    while projectile.position.y > 0.0 {
        projectile = tick(environment, projectile);

        let x = projectile.position.x as usize;
        let y = 550usize.saturating_sub(projectile.position.y as usize);

        canvas.write_pixel(x, y, red);
    }

    canvas
}

pub fn tick(environment: Environment, projectile: Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::{Point, Vector},
        scenes::projectile::{Environment, Projectile, tick},
    };

    #[test]
    fn tick_updates_position() {
        let projectile = Projectile {
            position: Point::new(1.0, 2.0, 3.0),
            velocity: Vector::new(4.0, 5.0, 6.0),
        };

        let environment = Environment {
            gravity: Vector::new(0.0, 0.0, 0.0),
            wind: Vector::new(0.0, 0.0, 0.0),
        };

        let result = tick(environment, projectile);

        assert_eq!(result.position, Point::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn tick_updates_velocity() {
        let projectile = Projectile {
            position: Point::new(1.0, 2.0, 3.0),
            velocity: Vector::new(4.0, 5.0, 6.0),
        };

        let environment = Environment {
            gravity: Vector::new(0.0, -0.1, 0.0),
            wind: Vector::new(-0.01, 0.0, 0.0),
        };

        let result = tick(environment, projectile);

        assert_eq!(result.velocity, Vector::new(3.99, 4.9, 6.0));
    }

    #[test]
    fn tick_updates_position_and_velocity() {
        let projectile = Projectile {
            position: Point::new(0.0, 1.0, 0.0),
            velocity: Vector::new(1.0, 1.0, 0.0),
        };

        let environment = Environment {
            gravity: Vector::new(0.0, -0.1, 0.0),
            wind: Vector::new(-0.01, 0.0, 0.0),
        };

        let result = tick(environment, projectile);

        assert_eq!(result.position, Point::new(1.0, 2.0, 0.0));
        assert_eq!(result.velocity, Vector::new(0.99, 0.9, 0.0));
    }
}
