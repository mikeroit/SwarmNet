use crate::model::Route;

#[derive(Debug, Clone, PartialEq)]
pub struct FlightPlan {
    pub id: String,
    pub mission_id: String,
    pub route: Route,
}

impl FlightPlan {
    pub fn new(id: impl Into<String>, mission_id: impl Into<String>, route: Route) -> FlightPlan {
        FlightPlan {
            id: id.into(),
            mission_id: mission_id.into(),
            route,
        }
    }
}
