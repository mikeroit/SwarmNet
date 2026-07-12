use std::ops::{Add, Sub};

use crate::math::Vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn direction_to(&self, other: &Self) -> Vector2 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let distance = self.distance_to(other);

        if distance == 0.0 {
            return Vector2::new(0.0, 0.0);
        }

        Vector2::new(dx / distance, dy / distance)
    }
}

impl Sub for Point2 {
    type Output = Vector2;

    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl Add<Vector2> for Point2 {
    type Output = Point2;

    fn add(self, v: Vector2) -> Self::Output {
        Self::new(self.x + v.x, self.y + v.y)
    }
}
