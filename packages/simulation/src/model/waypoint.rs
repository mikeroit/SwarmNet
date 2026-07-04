use crate::math::Vector2;

#[derive(Debug, Clone, PartialEq)]
pub struct Waypoint {
    pub id: String,
    pub position: Vector2,
}

impl Waypoint {
    pub fn new(id: impl Into<String>, position: Vector2) -> Self {
        Self {
            id: id.into(),
            position,
        }
    }
}
