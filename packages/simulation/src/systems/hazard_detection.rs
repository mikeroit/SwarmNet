use crate::{HazardObservation, events::DomainEvent, math::Circle, model::World};

pub struct HazardDetectionSystem;

impl HazardDetectionSystem {
    pub fn step(world: &mut World, observed_at_tick: u64) -> Vec<HazardObservation> {
        let hazards = world.hazards().to_vec();
        let mut events = Vec::new();
        let mut observations = vec![];

        for drone in world.drones_mut() {
            for hazard in &hazards {
                let sensor = Circle {
                    center: drone.position,
                    radius: drone.sensor_range_meters,
                };

                let can_detect = sensor.intersects_circle(&hazard.footprint);
                let already_known = drone.local_hazard_map.contains(&hazard.id);

                if can_detect && !already_known {
                    observations.push(HazardObservation::new(
                        drone.id.clone(),
                        hazard.clone(),
                        observed_at_tick,
                    ));

                    events.push(DomainEvent::HazardDetected {
                        drone_id: drone.id.clone(),
                        hazard_id: hazard.id.clone(),
                    });
                }
            }
        }

        for event in events {
            world.event_queue_mut().push(event);
        }

        observations
    }
}
