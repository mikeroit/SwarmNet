pub mod hazard_observation;
pub mod hazard_observation_transport;

pub use hazard_observation::HazardObservation;
pub use hazard_observation_transport::{
    HazardObservationTransport, InProcessHazardObservationTransport,
};
