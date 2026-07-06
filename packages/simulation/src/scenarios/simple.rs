use crate::math::Vector2;
use crate::model::{Route, RouteExecution, SimDrone, SimulationWorld, Waypoint};

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

        let route_execution = RouteExecution::new(route);

        let mut drone = SimDrone::new(
            "drone-001",
            Vector2::new(0.0, 0.0),
            10.0,
        );

        drone.assign_route_execution(route_execution);

        SimulationWorld::new(vec![drone])
    }
}
