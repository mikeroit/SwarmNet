use crate::math::Point2;
use crate::model::WaypointId;

#[derive(Debug, Clone, PartialEq)]
pub struct Waypoint {
    pub id: WaypointId,
    pub position: Point2,
}

impl Waypoint {
    pub fn new(id: impl Into<WaypointId>, position: Point2) -> Self {
        Self {
            id: id.into(),
            position,
        }
    }
}
