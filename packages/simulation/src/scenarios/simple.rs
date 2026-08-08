use crate::FlightPlan;
use crate::math::{Circle, Point2};
use crate::model::{Hazard, HazardSeverity, HazardState, HazardType};
use crate::model::{Mission, MissionAssignment, Route, SimDrone, Waypoint, World};

pub struct SimpleScenario;
pub struct MultiDroneScenario;

impl SimpleScenario {
    pub fn build() -> World {
        const MISSION_ID_STR: &str = "mid-001";

        let route = Route::new(
            "route-001".into(),
            vec![
                Waypoint::new("wp-001".into(), Point2::new(10.0, 0.0)),
                Waypoint::new("wp-002".into(), Point2::new(10.0, 10.0)),
                Waypoint::new("wp-003".into(), Point2::new(20.0, 10.0)),
            ],
        );

        let flight_plan = FlightPlan::new("fp-001".into(), MISSION_ID_STR.into(), route);

        let mut drone = SimDrone::new("drone-001".into(), Point2::new(0.0, 0.0), 10.0, 5.0);

        let mission_assignment = MissionAssignment::new(drone.id.clone(), flight_plan.id().clone());
        let mission = Mission::new(MISSION_ID_STR.into(), vec![mission_assignment]);

        drone.assign_flight_plan(flight_plan);

        World::new(vec![drone], vec![], vec![mission])
    }
}

impl MultiDroneScenario {
    pub fn build() -> World {
        const MISSION_ID_STR: &str = "mid-001";

        let route_a = Route::new(
            "route-001".into(),
            vec![
                Waypoint::new("wp-001".into(), Point2::new(10.0, 0.0)),
                Waypoint::new("wp-002".into(), Point2::new(10.0, 10.0)),
                Waypoint::new("wp-003".into(), Point2::new(20.0, 10.0)),
            ],
        );

        let route_b = Route::new(
            "route-002".into(),
            vec![
                Waypoint::new("wp-004".into(), Point2::new(0.0, 10.0)),
                Waypoint::new("wp-005".into(), Point2::new(2.0, 10.0)),
                Waypoint::new("wp-006".into(), Point2::new(15.0, 11.0)),
            ],
        );

        let route_c = Route::new(
            "route-003".into(),
            vec![
                Waypoint::new("wp-007".into(), Point2::new(5.0, 5.0)),
                Waypoint::new("wp-008".into(), Point2::new(10.0, 7.0)),
                Waypoint::new("wp-009".into(), Point2::new(20.0, 15.0)),
            ],
        );

        let flight_plan_a = FlightPlan::new("fp-001".into(), MISSION_ID_STR.into(), route_a);
        let flight_plan_b = FlightPlan::new("fp-002".into(), MISSION_ID_STR.into(), route_b);
        let flight_plan_c = FlightPlan::new("fp-003".into(), MISSION_ID_STR.into(), route_c);

        let mut drone_a = SimDrone::new("drone-001".into(), Point2::new(0.0, 0.0), 10.0, 5.0);
        let mut drone_b = SimDrone::new("drone-002".into(), Point2::new(5.0, 5.0), 10.0, 5.0);
        let mut drone_c = SimDrone::new("drone-003".into(), Point2::new(1.0, 2.0), 15.0, 5.0);

        let hazard_a = Hazard::new(
            "hazard-001".into(),
            Circle::new(Point2::new(2.0, 0.0), 1.0),
            HazardType::StaticObstacle,
            HazardSeverity::Low,
            HazardState::Active,
        );

        let hazards = vec![hazard_a];

        let mission_assignment_a =
            MissionAssignment::new(drone_a.id.clone(), flight_plan_a.id().clone());
        let mission_assignment_b =
            MissionAssignment::new(drone_b.id.clone(), flight_plan_b.id().clone());
        let mission_assignment_c =
            MissionAssignment::new(drone_c.id.clone(), flight_plan_c.id().clone());

        let mission_assignments = vec![
            mission_assignment_a,
            mission_assignment_b,
            mission_assignment_c,
        ];

        let mission = Mission::new(MISSION_ID_STR.into(), mission_assignments);

        drone_a.assign_flight_plan(flight_plan_a);
        drone_b.assign_flight_plan(flight_plan_b);
        drone_c.assign_flight_plan(flight_plan_c);

        let drones = vec![drone_a, drone_b, drone_c];

        World::new(drones, hazards, vec![mission])
    }
}
