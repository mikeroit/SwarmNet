use std::time::Duration;

use crate::model::{Route, RouteExecution, SimDrone, Waypoint};
use crate::math::Vector2;
use crate::systems::RouteFollowingSystem;

#[derive(Debug, Clone, PartialEq)]
pub struct SimulationWorld {
    drones: Vec<SimDrone>,
}

impl SimulationWorld {
    pub fn new() -> Self {
        Self { drones: Vec::new() }
    }

    pub fn with_default_drone() -> Self {
        let mut drone = SimDrone::new(
            "drone-001",
            Vector2::new(0.0, 0.0),
            10.0,
        );

        let route = Route::new(
            "route-001",
            vec![
                Waypoint::new("wp-001", Vector2::new(10.0, 0.0)),
                Waypoint::new("wp-002", Vector2::new(10.0, 10.0)),
                Waypoint::new("wp-003", Vector2::new(20.0, 10.0)),
            ],
        );

        drone.assign_route_execution(RouteExecution::new(
            route,
        ));

        Self {
            drones: vec![drone],
        }
    }

    pub fn drones(&self) -> &[SimDrone] {
        &self.drones
    }

    pub fn drones_mut(&mut self) -> &mut [SimDrone] {
        &mut self.drones
    }

    pub fn update(&mut self, tick_duration: Duration) {
        RouteFollowingSystem::step(self, tick_duration);
    }
}

impl Default for SimulationWorld {
    fn default() -> Self {
        Self::new()
    }
}
