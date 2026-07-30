use crate::FlightPlan;
use crate::math::{Circle, Point2};
use crate::model::{Hazard, HazardSeverity, HazardState, HazardType};
use crate::model::{Route, SimDrone, World, Waypoint};

pub struct SimpleScenario;
pub struct MultiDroneScenario;

impl SimpleScenario {
    pub fn build() -> World {
        let route = Route::new(
            "route-001".into(),
            vec![
                Waypoint::new("wp-001".into(), Point2::new(10.0, 0.0)),
                Waypoint::new("wp-002".into(), Point2::new(10.0, 10.0)),
                Waypoint::new("wp-003".into(), Point2::new(20.0, 10.0)),
            ],
        );

        let flight_plan = FlightPlan::new("fp-001".into(), "mid-001".into(), route);

        let mut drone = SimDrone::new("drone-001".into(), Point2::new(0.0, 0.0), 10.0, 5.0);

        drone.assign_flight_plan(flight_plan);

        World::new(vec![drone], vec![])
    }
}

impl MultiDroneScenario {
    pub fn build() -> World {
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

        let flight_plan_a = FlightPlan::new("fp-001".into(), "mission-001".into(), route_a);
        let flight_plan_b = FlightPlan::new("fp-002".into(), "mission-002".into(), route_b);
        let flight_plan_c = FlightPlan::new("fp-003".into(), "mission-003".into(), route_c);

        let mut drone_a = SimDrone::new("drone-001".into(), Point2::new(0.0, 0.0), 10.0, 5.0);
        let mut drone_b = SimDrone::new("drone-002".into(), Point2::new(5.0, 5.0), 10.0, 5.0);
        let mut drone_c = SimDrone::new("drone-003".into(), Point2::new(1.0, 2.0), 15.0, 5.0);

        drone_a.assign_flight_plan(flight_plan_a);
        drone_b.assign_flight_plan(flight_plan_b);
        drone_c.assign_flight_plan(flight_plan_c);

        let drones = vec![drone_a, drone_b, drone_c];

        let hazard_a = Hazard::new(
            "hazard-001".into(),
            Circle::new(Point2::new(2.0, 0.0), 1.0),
            HazardType::StaticObstacle,
            HazardSeverity::Low,
            HazardState::Active,
        );

        let hazards = vec![hazard_a];

        World::new(drones, hazards)
    }
}
