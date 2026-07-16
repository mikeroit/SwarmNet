use std::time::Duration;

use crate::{ExecutionStatus, SimulationEvent, SimulationWorld};

pub struct RouteFollowingSystem;

impl RouteFollowingSystem {
    pub fn step(world: &mut SimulationWorld, tick_duration: Duration) {
        let delta_seconds = tick_duration.as_secs_f64();

        let mut events = Vec::new();

        for drone in world.drones_mut() {
            let Some(flight_plan_execution) = drone.flight_plan_execution.as_mut() else {
                continue;
            };

            let Some(target_waypoint) = flight_plan_execution.route_execution.current_waypoint()
            else {
                continue;
            };

            if flight_plan_execution.execution_status == ExecutionStatus::Pending {
                flight_plan_execution.execution_status = ExecutionStatus::Active;
            }

            let drone_id = drone.id.clone();
            let route_id = flight_plan_execution.flight_plan.route.id.clone();
            let waypoint_id = target_waypoint.id.clone();
            let flight_plan_id = flight_plan_execution.flight_plan.id.clone();

            let target_position = target_waypoint.position;
            let distance_to_target = drone.position.distance_to(&target_position);
            let max_travel_distance = drone.speed_mps * delta_seconds;

            if distance_to_target <= max_travel_distance {
                events.push(SimulationEvent::WaypointReached {
                    drone_id: drone_id.clone(),
                    route_id: route_id.clone(),
                    waypoint_id: waypoint_id.clone(),
                });

                drone.position = target_position;

                flight_plan_execution.route_execution.advance_waypoint();

                if flight_plan_execution.route_execution.completed {
                    events.push(SimulationEvent::RouteCompleted {
                        drone_id: drone_id.clone(),
                        route_id: route_id.clone(),
                    });

                    events.push(SimulationEvent::FlightPlanCompleted {
                        drone_id: drone_id.clone(),
                        flight_plan_id: flight_plan_id.clone(),
                    });

                    flight_plan_execution.execution_status = ExecutionStatus::Completed;
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
