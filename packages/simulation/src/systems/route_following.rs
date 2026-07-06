use std::time::Duration;

use crate::SimulationWorld;

pub struct RouteFollowingSystem;

impl RouteFollowingSystem {
    pub fn step(world: &mut SimulationWorld, tick_duration: Duration) {
        let delta_seconds = tick_duration.as_secs_f64();

        for drone in world.drones_mut() {
            let Some(flight_plan_execution) = drone.flight_plan_execution.as_mut() else {
                continue;
            };

            let Some(target_waypoint) = flight_plan_execution.route_execution.current_waypoint() else {
                continue;
            };

            let target_position = target_waypoint.position;
            let distance_to_target = drone.position.distance_to(&target_position);
            let max_travel_distance = drone.speed_mps * delta_seconds;

            if distance_to_target <= max_travel_distance {
                drone.position = target_position;
                flight_plan_execution.route_execution.advance_waypoint();
            } else {
                let direction = drone.position.direction_to(&target_position);
                drone.position = drone.position.add_scaled(&direction, max_travel_distance);
            }
        }
    }
}
