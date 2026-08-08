use derive_new::new;

use crate::{DroneId, FlightPlanId, MissionId};

#[derive(Debug, Clone, PartialEq, new)]
pub struct MissionAssignment {
    drone_id: DroneId,
    flight_plan_id: FlightPlanId,
}

impl MissionAssignment {
    pub fn drone_id(&self) -> &DroneId {
        &self.drone_id
    }

    pub fn flight_plan_id(&self) -> &FlightPlanId {
        &self.flight_plan_id
    }
}

#[derive(Debug, Clone, PartialEq, new)]
pub struct Mission {
    id: MissionId,
    assignments: Vec<MissionAssignment>,
}

impl Mission {
    pub fn id(&self) -> &MissionId {
        &self.id
    }

    pub fn assignments(&self) -> &[MissionAssignment] {
        &self.assignments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FlightPlan, Point2, Route, SimDrone, Waypoint};

    #[test]
    fn new_preserves_drone_definition_and_flight_plan_definition() {
        const MISSION_ID_STR: &str = "mission-123";

        // routes
        let route_a = Route::new(
            "route-001".into(),
            vec![
                Waypoint::new("wp-001".into(), Point2::new(0.0, 0.0)),
                Waypoint::new("wp-002".into(), Point2::new(10.0, 0.0)),
            ],
        );

        let route_b = Route::new(
            "route-002".into(),
            vec![
                Waypoint::new("wp-003".into(), Point2::new(15.0, 15.0)),
                Waypoint::new("wp-004".into(), Point2::new(20.0, 20.0)),
            ],
        );

        // flight plans
        let flight_plan_a = FlightPlan::new(
            "flight-plan-001".into(),
            MISSION_ID_STR.into(),
            route_a.clone(),
        );

        let flight_plan_b = FlightPlan::new(
            "flight-plan-002".into(),
            MISSION_ID_STR.into(),
            route_b.clone(),
        );

        // drones
        let mut drone_a = SimDrone::new("drone-001".into(), Point2::new(0.0, 0.0), 5.0, 5.0);
        drone_a.assign_flight_plan(flight_plan_a);

        let mut drone_b = SimDrone::new("drone-002".into(), Point2::new(5.0, 5.0), 3.0, 4.0);
        drone_b.assign_flight_plan(flight_plan_b);

        // mission assignments
        let assignments = vec![
            MissionAssignment::new(
                drone_a.id.clone(),
                drone_a
                    .flight_plan_execution
                    .expect("expected to find fp initialized")
                    .flight_plan()
                    .id()
                    .clone(),
            ),
            MissionAssignment::new(
                drone_b.id.clone(),
                drone_b
                    .flight_plan_execution
                    .expect("expected to find fp initialized")
                    .flight_plan()
                    .id()
                    .clone(),
            ),
        ];

        // mission
        let mission = Mission::new(MISSION_ID_STR.into(), assignments);

        assert_eq!(mission.assignments().len(), 2);

        assert_eq!(
            mission.assignments()[0].drone_id().clone(),
            DroneId::from("drone-001")
        );
        assert_eq!(
            mission.assignments()[1].drone_id().clone(),
            DroneId::from("drone-002")
        );

        assert_eq!(
            mission.assignments()[0].flight_plan_id().clone(),
            FlightPlanId::from("flight-plan-001")
        );
        assert_eq!(
            mission.assignments()[1].flight_plan_id().clone(),
            FlightPlanId::from("flight-plan-002")
        );
    }
}
