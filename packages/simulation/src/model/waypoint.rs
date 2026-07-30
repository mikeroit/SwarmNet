use derive_new::new;

use crate::math::Point2;
use crate::model::WaypointId;

#[derive(Debug, Clone, PartialEq, new)]
pub struct Waypoint {
    pub id: WaypointId,
    pub position: Point2,
}

