use std::time::Duration;

use crate::{SimDrone, Vector2};

#[derive(Debug, Clone, PartialEq)]
pub struct SimulationWorld {
    drones: Vec<SimDrone>,
}

impl SimulationWorld {
    pub fn new() -> Self {
        Self { drones: Vec::new() }
    }

    pub fn with_default_drone() -> Self {
        Self {
            drones: vec![SimDrone::new(
                "drone-001",
                Vector2::new(0.0, 0.0),
                10.0,
            )],
        }
    }

    pub fn drones(&self) -> &[SimDrone] {
        &self.drones
    }

    pub fn update(&mut self, tick_duration: Duration) {
        let delta_seconds = tick_duration.as_secs_f64();

        for drone in &mut self.drones {
            drone.update(delta_seconds);
        }
    }
}

impl Default for SimulationWorld {
    fn default() -> Self {
        Self::new()
    }
}
