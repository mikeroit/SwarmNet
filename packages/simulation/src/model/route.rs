use crate::Waypoint;

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub id: String,
    pub waypoints: Vec<Waypoint>,
}

impl Route {
    pub fn new(id: impl Into<String>, waypoints: Vec<Waypoint>) -> Self {
        Self {
            id: id.into(),
            waypoints,
        }
    }
}
