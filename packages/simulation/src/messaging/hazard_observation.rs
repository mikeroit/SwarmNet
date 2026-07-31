use derive_new::new;

use crate::{DroneId, Hazard};

#[derive(Debug, Clone, PartialEq, new)]
pub struct HazardObservation {
    source_drone_id: DroneId,
    hazard: Hazard,
    observed_at_tick: u64,
}

impl HazardObservation {
    pub fn source_drone_id(&self) -> &DroneId {
        &self.source_drone_id
    }

    pub fn hazard(&self) -> &Hazard {
        &self.hazard
    }

    pub fn observed_at_tick(&self) -> u64 {
        self.observed_at_tick
    }
}
