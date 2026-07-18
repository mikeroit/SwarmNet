use crate::{LineSegment, RouteId, Waypoint};

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    id: RouteId,
    waypoints: Vec<Waypoint>,
}

impl Route {
    pub fn new(id: impl Into<RouteId>, waypoints: Vec<Waypoint>) -> Self {
        Self {
            id: id.into(),
            waypoints,
        }
    }

    pub fn id(&self) -> &RouteId {
        &self.id
    }

    pub fn segments(&self) -> Vec<LineSegment> {
        self.waypoints
            .windows(2)
            .map(|pair| LineSegment::new(pair[0].position, pair[1].position))
            .collect()
    }

    pub fn waypoints(&self) -> &[Waypoint] {
        &self.waypoints
    }
}

