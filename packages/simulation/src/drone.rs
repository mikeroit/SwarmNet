use crate::Vector2;

#[derive(Debug, Clone, PartialEq)]
pub struct SimDrone {
    pub id: String,
    pub position: Vector2,
    pub speed_mps: f64,
}

impl SimDrone {
    pub fn new(id: impl Into<String>, position: Vector2, speed_mps: f64) -> Self {
        Self {
            id: id.into(),
            position,
            speed_mps,
        }
    }

    pub fn update(&mut self, delta_seconds: f64) {
        self.position.x += self.speed_mps * delta_seconds;
    }
}
