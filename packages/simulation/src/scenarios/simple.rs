use crate::FlightPlan;
use crate::math::Point2;
use crate::model::{Hazard, HazardSeverity, HazardState, HazardType};
use crate::model::{Route, SimDrone, SimulationWorld, Waypoint};

pub struct SimpleScenario;
pub struct MultiDroneScenario;

impl SimpleScenario {
    pub fn build() -> SimulationWorld {
        let route = Route::new(
            "route-001",
            vec![
                Waypoint::new("wp-001", Point2::new(10.0, 0.0)),
                Waypoint::new("wp-002", Point2::new(10.0, 10.0)),
                Waypoint::new("wp-003", Point2::new(20.0, 10.0)),
            ],
        );

        let flight_plan = FlightPlan::new("fp-001", "mid-001", route);

        let mut drone = SimDrone::new("drone-001", Point2::new(0.0, 0.0), 10.0);

        drone.assign_flight_plan(flight_plan);

        SimulationWorld::new(vec![drone], vec![])
    }
}

impl MultiDroneScenario {
    pub fn build() -> SimulationWorld {
        let route_a = Route::new(
            "route-001",
            vec![
                Waypoint::new("wp-001", Point2::new(10.0, 0.0)),
                Waypoint::new("wp-002", Point2::new(10.0, 10.0)),
                Waypoint::new("wp-003", Point2::new(20.0, 10.0)),
            ],
        );

        let route_b = Route::new(
            "route-002",
            vec![
                Waypoint::new("wp-004", Point2::new(0.0, 10.0)),
                Waypoint::new("wp-005", Point2::new(2.0, 10.0)),
                Waypoint::new("wp-006", Point2::new(15.0, 11.0)),
            ],
        );

        let route_c = Route::new(
            "route-003",
            vec![
                Waypoint::new("wp-007", Point2::new(5.0, 5.0)),
                Waypoint::new("wp-008", Point2::new(10.0, 7.0)),
                Waypoint::new("wp-009", Point2::new(20.0, 15.0)),
            ],
        );

        let flight_plan_a = FlightPlan::new("fp-001", "mission-001", route_a);
        let flight_plan_b = FlightPlan::new("fp-002", "mission-002", route_b);
        let flight_plan_c = FlightPlan::new("fp-003", "mission-003", route_c);

        let mut drone_a = SimDrone::new("drone-001", Point2::new(0.0, 0.0), 10.0);
        let mut drone_b = SimDrone::new("drone-002", Point2::new(5.0, 5.0), 10.0);
        let mut drone_c = SimDrone::new("drone-003", Point2::new(1.0, 2.0), 15.0);

        drone_a.assign_flight_plan(flight_plan_a);
        drone_b.assign_flight_plan(flight_plan_b);
        drone_c.assign_flight_plan(flight_plan_c);

        let drones = vec![drone_a, drone_b, drone_c];

        let hazard_a = Hazard::new(
            "hazard-001".into(),
            Point2::new(2.0, 5.0),
            1.0,
            HazardType::StaticObstacle,
            HazardSeverity::Low,
            HazardState::Active,
        );
        let hazard_b = Hazard::new(
            "hazard-002".into(),
            Point2::new(2.0, 17.0),
            2.0,
            HazardType::NoFlyZone,
            HazardSeverity::High,
            HazardState::Cleared,
        );

        let hazards = vec![hazard_a, hazard_b];

        SimulationWorld::new(drones, hazards)
    }
}
