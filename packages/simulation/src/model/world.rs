use std::time::Duration;

use crate::model::{SimDrone};
use crate::systems::RouteFollowingSystem;

#[derive(Debug, Clone, PartialEq)]
pub struct SimulationWorld {
    drones: Vec<SimDrone>,
}

impl SimulationWorld {
    pub fn new(drones: Vec<SimDrone>) -> Self {
        Self { drones }
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
        Self::new(Vec::new())
    }
}
