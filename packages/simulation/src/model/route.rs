use crate::RouteId;
use crate::Waypoint;

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub id: RouteId,
    pub waypoints: Vec<Waypoint>,
}

impl Route {
    pub fn new(id: impl Into<RouteId>, waypoints: Vec<Waypoint>) -> Self {
        Self {
            id: id.into(),
            waypoints,
        }
    }
}
