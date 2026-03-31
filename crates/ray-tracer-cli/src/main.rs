use ray_tracer_core::{
    math::{Point, Vector},
    projectile::{Environment, Projectile, tick},
};

fn main() {
    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let mut tick_count = 0;

    while projectile.position.y > 0.0 {
        println!(
            "tick {:>3}: position=({:.2}, {:.2}, {:.2})",
            tick_count, projectile.position.x, projectile.position.y, projectile.position.z
        );

        projectile = tick(environment, projectile);
        tick_count += 1;
    }

    println!("Projectile landed after {tick_count} ticks.");
}
