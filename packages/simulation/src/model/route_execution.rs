use crate::Route;
use crate::Waypoint;

#[derive(Debug, Clone, PartialEq)]
pub struct RouteExecution {
    pub route: Route,
    pub current_waypoint_index: usize,
    pub completed: bool,
}

impl RouteExecution {
    pub fn new(route: Route) -> Self {
        Self {
            route,
            current_waypoint_index: 0,
            completed: false,
        }
    }

    pub fn current_waypoint(&self) -> Option<&Waypoint> {
        if self.completed {
            return None;
        }

        self.route.waypoints.get(self.current_waypoint_index)
    }

    pub fn advance_waypoint(&mut self) {
        if self.current_waypoint_index + 1 >= self.route.waypoints.len() {
            self.completed = true;
        } else {
            self.current_waypoint_index += 1;
        }
    }
}
