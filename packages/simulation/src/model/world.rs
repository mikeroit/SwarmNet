use std::time::Duration;
use derive_new::new;

use crate::events::{EventQueue, DomainEvent};
use crate::model::{Hazard, SimDrone};
use crate::systems::{
    HazardDetectionSystem, RouteFollowingSystem, RoutePlanningSystem, RouteValidationSystem,
};

#[derive(Debug, Clone, PartialEq, new)]
pub struct World {
    drones: Vec<SimDrone>,
    #[new(value = "EventQueue::new(vec!())")]
    event_queue: EventQueue,
    hazards: Vec<Hazard>,
}

impl World {
    pub fn drones(&self) -> &[SimDrone] {
        &self.drones
    }

    pub fn drones_mut(&mut self) -> &mut [SimDrone] {
        &mut self.drones
    }

    pub fn event_queue_mut(&mut self) -> &mut EventQueue {
        &mut self.event_queue
    }

    pub fn drain_events(&mut self) -> Vec<DomainEvent> {
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
        RoutePlanningSystem::step(self);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(Vec::new(), Vec::new())
    }
}
