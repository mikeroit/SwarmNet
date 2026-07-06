#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn direction_to(&self, other: &Self) -> Self {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let distance = self.distance_to(other);

        if distance == 0.0 {
            return Self::new(0.0, 0.0);
        }

        Self::new(dx / distance, dy / distance)
    }

    pub fn add_scaled(&self, direction: &Self, scale: f64) -> Self {
        Self::new(self.x + direction.x * scale, self.y + direction.y * scale)
    }
}
