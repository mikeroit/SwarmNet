use derive_new::new;

use crate::math::Point2;
use crate::model::WaypointId;

#[derive(Debug, Clone, PartialEq, new)]
pub struct Waypoint {
    id: WaypointId,
    position: Point2,
}

impl Waypoint {
    pub fn id(&self) -> &WaypointId {
        &self.id
    }

    pub fn position(&self) -> Point2 {
        self.position
    }
}
