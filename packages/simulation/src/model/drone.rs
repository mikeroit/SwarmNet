use crate::math::Vector2;
use crate::model::Route;

#[derive(Debug, Clone, PartialEq)]
pub struct SimDrone {
    pub id: String,
    pub position: Vector2,
    pub speed_mps: f64,
    pub route: Option<Route>,
}

impl SimDrone {
    pub fn new(id: impl Into<String>, position: Vector2, speed_mps: f64) -> Self {
        Self {
            id: id.into(),
            position,
            speed_mps,
            route: None,
        }
    }

    pub fn assign_route(&mut self, route: Route) {
        self.route = Some(route);
    }
}
