use crate::model::{FlightPlanId, MissionId, Route};

#[derive(Debug, Clone, PartialEq)]
pub struct FlightPlan {
    id: FlightPlanId,
    mission_id: MissionId,
    route: Route,
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

    pub fn id(&self) -> &FlightPlanId {
        &self.id
    }

    pub fn mission_id(&self) -> &MissionId {
        &self.mission_id
    }

    pub fn route(&self) -> &Route {
        &self.route
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point2, Waypoint};

    #[test]
    fn new_preserves_flight_plan_definition() {
        let route = Route::new(
            "route-001",
            vec![
                Waypoint::new("wp-001", Point2::new(0.0, 0.0)),
                Waypoint::new("wp-002", Point2::new(10.0, 0.0)),
            ],
        );

        let flight_plan = FlightPlan::new(
            "flight-plan-001",
            "mission-001",
            route.clone(),
        );

        assert_eq!(
            flight_plan.id(),
            &FlightPlanId::from("flight-plan-001")
        );

        assert_eq!(
            flight_plan.mission_id(),
            &MissionId::from("mission-001")
        );

        assert_eq!(flight_plan.route(), &route);
    }
}
