use crate::Route;
use crate::Waypoint;

#[derive(Debug, Clone, PartialEq)]
pub struct RouteExecution {
    route: Route,
    current_waypoint_index: usize,
    completed: bool,
}

impl RouteExecution {
    pub fn current_waypoint(&self) -> Option<&Waypoint> {
        if self.completed {
            return None;
        }

        self.route.waypoints().get(self.current_waypoint_index)
    }

    pub fn advance_waypoint(&mut self) {
        if self.current_waypoint_index + 1 >= self.route.waypoints().len() {
            self.completed = true;
        } else {
            self.current_waypoint_index += 1;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.completed
    }

    pub fn active_route(&self) -> &Route {
        &self.route
    }

    pub fn current_waypoint_index(&self) -> usize {
        self.current_waypoint_index
    }
}

impl From<Route> for RouteExecution {
    fn from(route: Route) -> Self {
        Self {
            route,
            current_waypoint_index: 0,
            completed: false,
        }
    }
}
