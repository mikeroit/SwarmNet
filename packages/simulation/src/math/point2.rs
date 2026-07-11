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

    pub fn translate(&self, vector: Vector2) -> Self {
        Self::new(self.x + vector.x, self.y + vector.y)
    }

    pub fn add_scaled(&self, direction: &Vector2, scale: f64) -> Self {
        Self::new(self.x + direction.x * scale, self.y + direction.y * scale)
    }
}
