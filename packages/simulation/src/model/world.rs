use std::time::Duration;

use crate::model::{SimDrone};
use crate::events::{EventQueue, SimulationEvent};
use crate::systems::RouteFollowingSystem;

#[derive(Debug, Clone, PartialEq)]
pub struct SimulationWorld {
    drones: Vec<SimDrone>,
    event_queue: EventQueue,
}

impl SimulationWorld {
    pub fn new(drones: Vec<SimDrone>) -> Self {
        Self {
            drones,
            event_queue: EventQueue::new(),
        }
    }

    pub fn drones(&self) -> &[SimDrone] {
        &self.drones
    }

    pub fn drones_mut(&mut self) -> &mut [SimDrone] {
        &mut self.drones
    }

    pub fn event_queue(&self) -> &EventQueue {
        &self.event_queue
    }

    pub fn event_queue_mut(&mut self) -> &mut EventQueue {
        &mut self.event_queue
    }

    pub fn drain_events(&mut self) -> Vec<SimulationEvent> {
        self.event_queue.drain()
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
