use derive_new::new;

use crate::Route;
use crate::Waypoint;

#[derive(Debug, Clone, PartialEq, new)]
pub struct RouteExecution {
    route: Route,
    #[new(value = "0")]
    current_waypoint_index: usize,
    #[new(value = "false")]
    completed: bool,
}

impl RouteExecution {
    pub fn current_waypoint(&self) -> Option<&Waypoint> {
        if self.completed {
            return None;
        }

        self.route
            .waypoints()
            .get(self.current_waypoint_index)
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
