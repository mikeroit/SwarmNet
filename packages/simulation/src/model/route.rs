use crate::Waypoint;

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub id: String,
    pub waypoints: Vec<Waypoint>,
    pub current_waypoint_index: usize,
    pub completed: bool,
}

impl Route {
    pub fn new(id: impl Into<String>, waypoints: Vec<Waypoint>) -> Self {
        Self {
            id: id.into(),
            waypoints,
            current_waypoint_index: 0,
            completed: false,
        }
    }

    pub fn current_waypoint(&self) -> Option<&Waypoint> {
        if self.completed {
            return None;
        }

        self.waypoints.get(self.current_waypoint_index)
    }

    pub fn advance_waypoint(&mut self) {
        if self.current_waypoint_index + 1 >= self.waypoints.len() {
            self.completed = true;
        } else {
            self.current_waypoint_index += 1;
        }
    }
}
