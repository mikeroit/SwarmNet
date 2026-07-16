use std::time::Duration;

use crate::events::{EventQueue, SimulationEvent};
use crate::model::{Hazard, SimDrone};
use crate::systems::{HazardDetectionSystem, RouteFollowingSystem, RouteValidationSystem};

#[derive(Debug, Clone, PartialEq)]
pub struct SimulationWorld {
    drones: Vec<SimDrone>,
    event_queue: EventQueue,
    hazards: Vec<Hazard>,
}

impl SimulationWorld {
    pub fn new(drones: Vec<SimDrone>, hazards: Vec<Hazard>) -> Self {
        Self {
            drones,
            event_queue: EventQueue::new(),
            hazards,
        }
    }

    pub fn drones(&self) -> &[SimDrone] {
        &self.drones
    }

    pub fn drones_mut(&mut self) -> &mut [SimDrone] {
        &mut self.drones
    }

    pub fn event_queue_mut(&mut self) -> &mut EventQueue {
        &mut self.event_queue
    }

    pub fn drain_events(&mut self) -> Vec<SimulationEvent> {
        self.event_queue.drain()
    }

    pub fn hazards(&self) -> &[Hazard] {
        &self.hazards
    }

    pub fn hazards_mut(&mut self) -> &mut [Hazard] {
        &mut self.hazards
    }

    pub fn update(&mut self, tick_duration: Duration) {
        RouteFollowingSystem::step(self, tick_duration);
        HazardDetectionSystem::step(self);
        RouteValidationSystem::step(self);
    }
}

impl Default for SimulationWorld {
    fn default() -> Self {
        Self::new(Vec::new(), Vec::new())
    }
}
