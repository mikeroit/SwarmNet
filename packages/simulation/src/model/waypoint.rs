use crate::math::Vector2;
use crate::model::WaypointId;

#[derive(Debug, Clone, PartialEq)]
pub struct Waypoint {
    pub id: WaypointId,
    pub position: Vector2,
}

impl Waypoint {
    pub fn new(id: impl Into<WaypointId>, position: Vector2) -> Self {
        Self {
            id: id.into(),
            position,
        }
    }
}
