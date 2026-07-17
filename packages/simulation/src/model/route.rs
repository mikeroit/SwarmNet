use crate::{LineSegment, RouteId, Waypoint};

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

    pub fn segments(&self) -> Vec<LineSegment> {
        self.waypoints
            .windows(2)
            .map(|pair| LineSegment::new(pair[0].position, pair[1].position))
            .collect()
    }

    pub fn waypoints(&self) -> &[Waypoint] {
        self.waypoints.as_slice()
    }

    pub fn push_waypoint(&mut self, waypoint: Waypoint) {
        self.waypoints.push(waypoint);
    }

    pub fn extend(&mut self, mut other: Route) {
        if self.waypoints.is_empty() && !other.waypoints.is_empty() {
            other.waypoints.remove(0);
        }

        self.waypoints.extend(other.waypoints);
    }
}
