use derive_new::new;

use crate::model::{FlightPlanId, MissionId, Route};

#[derive(Debug, Clone, PartialEq, new)]
pub struct FlightPlan {
    id: FlightPlanId,
    mission_id: MissionId,
    route: Route,
}

impl FlightPlan {
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
            "route-001".into(),
            vec![
                Waypoint::new("wp-001".into(), Point2::new(0.0, 0.0)),
                Waypoint::new("wp-002".into(), Point2::new(10.0, 0.0)),
            ],
        );

        let flight_plan = FlightPlan::new(
            "flight-plan-001".into(),
            "mission-001".into(),
            route.clone(),
        );

        assert_eq!(flight_plan.id(), &FlightPlanId::from("flight-plan-001"));

        assert_eq!(flight_plan.mission_id(), &MissionId::from("mission-001"));

        assert_eq!(flight_plan.route(), &route);
    }
}
