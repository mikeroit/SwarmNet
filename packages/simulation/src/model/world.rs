use derive_new::new;
use std::time::Duration;

use crate::events::{DomainEvent, EventQueue};
use crate::messaging::HazardObservationTransport;
use crate::model::{Hazard, SimDrone};
use crate::systems::{
    HazardDetectionSystem, HazardSharingSystem, RouteFollowingSystem, RoutePlanningSystem,
    RouteValidationSystem,
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

    pub fn update(
        &mut self,
        tick_duration: Duration,
        current_tick: u64,
        hazard_transport: &mut dyn HazardObservationTransport,
    ) {
        // move drones along routes
        RouteFollowingSystem::step(self, tick_duration);

        //each drone detects hazards
        let observations = HazardDetectionSystem::step(self, current_tick);

        for observation in observations {
            hazard_transport.publish(observation);
        }

        let recieved_observations = hazard_transport.drain();

        // share hazards to all drones
        HazardSharingSystem::step(self, &recieved_observations);

        // drones validate routes based on new hazard map update
        RouteValidationSystem::step(self);

        // drones replan routes if needed
        RoutePlanningSystem::step(self);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(Vec::new(), Vec::new())
    }
}
