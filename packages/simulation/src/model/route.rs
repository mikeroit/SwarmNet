#[cfg(test)]
use crate::Point2;
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
        if !self.waypoints.is_empty() && !other.waypoints.is_empty() {
            other.waypoints.remove(0);
        }

        self.waypoints.extend(other.waypoints);
    }
}

#[test]
fn extend_preserves_first_waypoint_and_deduplicates_boundary() {
    let a = Waypoint::new("a", Point2::new(0.0, 0.0));
    let b = Waypoint::new("b", Point2::new(1.0, 0.0));
    let c = Waypoint::new("c", Point2::new(2.0, 0.0));

    let mut route = Route::new("route", vec![a.clone(), b.clone()]);
    route.extend(Route::new("segment", vec![b.clone(), c.clone()]));

    assert_eq!(route.waypoints(), &[a, b, c]);
}
