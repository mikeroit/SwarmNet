use crate::events::SimulationEvent;
use crate::model::{Hazard, RoutePlanner, SimulationWorld, ValidationStatus, Waypoint};

pub struct RoutePlanningSystem;

impl RoutePlanningSystem {
    pub fn step(world: &mut SimulationWorld) {
        let mut events = Vec::new();

        for drone in world.drones_mut() {
            let drone_id = drone.id.clone();
            let drone_position = drone.position;

            let Some(flight_plan_execution) = drone.flight_plan_execution.as_mut() else {
                continue;
            };

            if flight_plan_execution.validation_status != ValidationStatus::Blocked {
                continue;
            }

            let route_execution = &flight_plan_execution.route_execution;

            let route_id = route_execution.route.id.clone();
            let current_index = route_execution.current_waypoint_index;

            let remaining_waypoints = &route_execution.route.waypoints()[current_index..];

            let mut planning_waypoints = Vec::with_capacity(remaining_waypoints.len() + 1);

            planning_waypoints.push(Waypoint::new(
                format!("replan-start-{drone_id}"),
                drone_position,
            ));

            planning_waypoints.extend_from_slice(remaining_waypoints);

            let hazards: Vec<&Hazard> = drone.local_hazard_map.hazards().collect();

            let new_route = RoutePlanner::plan(
                route_id,
                &planning_waypoints,
                &hazards
            );

            let new_route_id = new_route.id.clone();

            flight_plan_execution.replace_route(new_route);

            events.push(SimulationEvent::RouteReplanned {
                drone_id,
                route_id: new_route_id,
            });
        }

        for event in events {
            world.event_queue_mut().push(event);
        }
    }
}
