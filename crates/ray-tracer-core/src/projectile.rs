use crate::math::{Point, Vector};

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

pub fn tick(environment: Environment, projectile: Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}
