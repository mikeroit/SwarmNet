use crate::{HazardObservation, World};

pub struct HazardSharingSystem;

impl HazardSharingSystem {
    pub fn step(world: &mut World, observations: &[HazardObservation]) {
        for observation in observations {
            for drone in world.drones_mut() {
                drone.local_hazard_map.insert(observation.hazard().clone());
            }
        }
    }
}
