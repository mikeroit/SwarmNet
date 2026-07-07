use crate::FlightPlan;
use crate::math::Vector2;
use crate::model::{Route, SimDrone, SimulationWorld, Waypoint};

pub struct SimpleScenario;
pub struct MultiDroneScenario;

impl SimpleScenario {
    pub fn build() -> SimulationWorld {
        let route = Route::new(
            "route-001",
            vec![
                Waypoint::new("wp-001", Vector2::new(10.0, 0.0)),
                Waypoint::new("wp-002", Vector2::new(10.0, 10.0)),
                Waypoint::new("wp-003", Vector2::new(20.0, 10.0)),
            ],
        );

        let flight_plan = FlightPlan::new("fp-001", "mid-001", route);

        let mut drone = SimDrone::new("drone-001", Vector2::new(0.0, 0.0), 10.0);

        drone.assign_flight_plan(flight_plan);

        SimulationWorld::new(vec![drone])
    }
}

impl MultiDroneScenario {
    pub fn build() -> SimulationWorld {
        let route_a = Route::new(
            "route-001",
            vec![
                Waypoint::new("wp-001", Vector2::new(10.0, 0.0)),
                Waypoint::new("wp-002", Vector2::new(10.0, 10.0)),
                Waypoint::new("wp-003", Vector2::new(20.0, 10.0)),
            ],
        );

        let route_b = Route::new(
            "route-002",
            vec![
                Waypoint::new("wp-004", Vector2::new(0.0, 10.0)),
                Waypoint::new("wp-005", Vector2::new(2.0, 10.0)),
                Waypoint::new("wp-006", Vector2::new(15.0, 11.0)),
            ],
        );

        let route_c = Route::new(
            "route-003",
            vec![
                Waypoint::new("wp-007", Vector2::new(5.0, 5.0)),
                Waypoint::new("wp-008", Vector2::new(10.0, 7.0)),
                Waypoint::new("wp-009", Vector2::new(20.0, 15.0)),
            ],
        );

        let flight_plan_a = FlightPlan::new("fp-001", "mission-001", route_a);
        let flight_plan_b = FlightPlan::new("fp-002", "mission-002", route_b);
        let flight_plan_c = FlightPlan::new("fp-003", "mission-003", route_c);

        let mut drone_a = SimDrone::new("drone-001", Vector2::new(0.0, 0.0), 10.0);
        let mut drone_b = SimDrone::new("drone-002", Vector2::new(5.0, 5.0), 10.0);
        let mut drone_c = SimDrone::new("drone-003", Vector2::new(1.0, 2.0), 15.0);

        drone_a.assign_flight_plan(flight_plan_a);
        drone_b.assign_flight_plan(flight_plan_b);
        drone_c.assign_flight_plan(flight_plan_c);

        SimulationWorld::new(vec![drone_a, drone_b, drone_c])
    }
}
