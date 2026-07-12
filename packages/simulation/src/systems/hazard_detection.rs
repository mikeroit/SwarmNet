use crate::{events::SimulationEvent, math::Circle, model::SimulationWorld};

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

                let sensor = Circle {
                    center: drone.position,
                    radius: drone.sensor_range_meters,
                };

                if sensor.intersects_circle(&hazard.footprint) {
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
