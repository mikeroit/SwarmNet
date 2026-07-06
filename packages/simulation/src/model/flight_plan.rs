use crate::model::{FlightPlanId, MissionId, Route};

#[derive(Debug, Clone, PartialEq)]
pub struct FlightPlan {
    pub id: FlightPlanId,
    pub mission_id: MissionId,
    pub route: Route,
}

impl FlightPlan {
    pub fn new(
        id: impl Into<FlightPlanId>,
        mission_id: impl Into<MissionId>,
        route: Route,
    ) -> Self {
        Self {
            id: id.into(),
            mission_id: mission_id.into(),
            route,
        }
    }
}
