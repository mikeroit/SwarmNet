use crate::{events::DomainEvent, math::Circle, model::World};

pub struct HazardDetectionSystem;

impl HazardDetectionSystem {
    pub fn step(world: &mut World) {
        let hazards = world.hazards().to_vec();
        let mut events = Vec::new();

        for drone in world.drones_mut() {
            for hazard in &hazards {
                let sensor = Circle {
                    center: drone.position,
                    radius: drone.sensor_range_meters,
                };

                let can_detect = sensor.intersects_circle(&hazard.footprint);

                if can_detect && drone.local_hazard_map.insert(hazard.clone()) {
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
    }
}
