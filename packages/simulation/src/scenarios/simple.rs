use crate::math::Vector2;
use crate::model::{Route, SimDrone, SimulationWorld, Waypoint};
use crate::FlightPlan;

pub struct SimpleScenario;

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

        let mut drone = SimDrone::new(
            "drone-001",
            Vector2::new(0.0, 0.0),
            10.0,
        );

        drone.assign_flight_plan_execution(flight_plan);

        SimulationWorld::new(vec![drone])
    }
}
