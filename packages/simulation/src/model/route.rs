use crate::{LineSegment, RouteId, Waypoint};
use derive_new::new;

#[derive(Debug, Clone, PartialEq, new)]
pub struct Route {
    id: RouteId,
    waypoints: Vec<Waypoint>,
}

impl Route {
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

