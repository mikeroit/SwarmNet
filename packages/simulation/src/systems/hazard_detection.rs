use crate::{events::SimulationEvent, model::SimulationWorld};

pub struct HazardDetectionSystem;

impl HazardDetectionSystem {
    pub fn step(world: &mut SimulationWorld) {
        let hazards = world.hazards().to_vec();
        let mut events = Vec::new();

        for drone in world.drones_mut() {
            for hazard in &hazards {
                if drone.hazard_awareness.has_detected(&hazard.id) {
                    continue;
                }

                let detection_distance = drone.sensor_range_meters + hazard.footprint.radius;

                let distance = drone.position.distance_to(&hazard.footprint.center);

                if distance <= detection_distance {
                    drone.hazard_awareness.mark_detected(hazard.id.clone());

                    events.push(SimulationEvent::HazardDetected {
                        drone_id: drone.id.clone(),
                        hazard_id: hazard.id.clone(),
                    });
                }
            }
        }

        for event in events {
            world.event_queue_mut().push(event);
        }
    }
}
