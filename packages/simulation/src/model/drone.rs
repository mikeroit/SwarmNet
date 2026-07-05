use crate::math::Vector2;
use crate::model::RouteExecution;

#[derive(Debug, Clone, PartialEq)]
pub struct SimDrone {
    pub id: String,
    pub position: Vector2,
    pub speed_mps: f64,
    pub route_execution: Option<RouteExecution>,
}

impl SimDrone {
    pub fn new(id: impl Into<String>, position: Vector2, speed_mps: f64) -> Self {
        Self {
            id: id.into(),
            position,
            speed_mps,
            route_execution: None,
        }
    }

    pub fn assign_route_execution(&mut self, route_execution: RouteExecution) {
        self.route_execution = Some(route_execution);
    }
}
