use derive_new::new;
use std::collections::VecDeque;
use std::fmt::Debug;

use crate::HazardObservation;

pub trait HazardObservationTransport: Debug {
    fn publish(&mut self, observation: HazardObservation);

    fn drain(&mut self) -> Vec<HazardObservation>;
}

#[derive(Debug, Default, new)]
pub struct InProcessHazardObservationTransport {
    #[new(default)]
    observations: VecDeque<HazardObservation>,
}

impl HazardObservationTransport for InProcessHazardObservationTransport {
    fn publish(&mut self, observation: HazardObservation) {
        self.observations.push_back(observation);
    }

    fn drain(&mut self) -> Vec<HazardObservation> {
        self.observations.drain(..).collect()
    }
}
