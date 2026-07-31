use std::time::Duration;

use crate::{DomainEvent, ExecutionStatus, World};

pub struct RouteFollowingSystem;

impl RouteFollowingSystem {
    pub fn step(world: &mut World, tick_duration: Duration) {
        let delta_seconds = tick_duration.as_secs_f64();

        let mut events = Vec::new();

        for drone in world.drones_mut() {
            let Some(flight_plan_execution) = drone.flight_plan_execution.as_mut() else {
                continue;
            };

            let Some((waypoint_id, target_position)) = flight_plan_execution
                .route_execution()
                .current_waypoint()
                .map(|waypoint| (waypoint.id.clone(), waypoint.position))
            else {
                continue;
            };

            if flight_plan_execution.execution_status() == ExecutionStatus::Pending {
                flight_plan_execution.start_execution();
            }

            let drone_id = drone.id.clone();
            let route_id = flight_plan_execution
                .route_execution()
                .active_route()
                .id()
                .clone();
            let flight_plan_id = flight_plan_execution.flight_plan().id().clone();

            let distance_to_target = drone.position.distance_to(&target_position);
            let max_travel_distance = drone.speed_mps * delta_seconds;

            if distance_to_target <= max_travel_distance {
                events.push(DomainEvent::WaypointReached {
                    drone_id: drone_id.clone(),
                    route_id: route_id.clone(),
                    waypoint_id: waypoint_id.clone(),
                });

                drone.position = target_position;

                flight_plan_execution
                    .route_execution_mut()
                    .advance_waypoint();

                if flight_plan_execution.route_execution().is_complete() {
                    events.push(DomainEvent::RouteCompleted {
                        drone_id: drone_id.clone(),
                        route_id: route_id.clone(),
                    });

                    events.push(DomainEvent::FlightPlanCompleted {
                        drone_id: drone_id.clone(),
                        flight_plan_id: flight_plan_id.clone(),
                    });

                    flight_plan_execution.complete_execution();
                }
            } else {
                let direction = drone.position.direction_to(&target_position);
                drone.position = drone.position + (direction * max_travel_distance);
            }
        }

        for event in events.drain(..) {
            world.event_queue_mut().push(event);
        }
    }
}
