use crate::{
    events::SimulationEvent,
    math::LineSegment,
    model::{HazardId, HazardState, SimDrone, SimulationWorld, ValidationStatus},
};
#[cfg(test)]
use crate::model::{DroneId, RouteId};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RouteValidationResult {
    blocking_hazard_ids: Vec<HazardId>,
}

impl RouteValidationResult {
    pub fn new(blocking_hazard_ids: Vec<HazardId>) -> Self {
        Self {
            blocking_hazard_ids,
        }
    }

    pub fn valid() -> Self {
        Self::default()
    }

    pub fn is_valid(&self) -> bool {
        self.blocking_hazard_ids.is_empty()
    }

    pub fn blocking_hazard_ids(&self) -> &[HazardId] {
        &self.blocking_hazard_ids
    }
}

pub struct RouteValidationSystem;

impl RouteValidationSystem {
    /// Validates the remaining portion of a drone's active route against the
    /// hazards currently known by that drone.
    ///
    /// This function does not mutate the drone, emit events, or replan routes.
    pub fn validate(drone: &SimDrone) -> RouteValidationResult {
        let Some(flight_plan_execution) = drone.flight_plan_execution.as_ref() else {
            return RouteValidationResult::valid();
        };

        let route_execution = &flight_plan_execution.route_execution;

        if route_execution.completed {
            return RouteValidationResult::valid();
        }

        let remaining_segments = Self::remaining_route_segments(drone);

        if remaining_segments.is_empty() {
            return RouteValidationResult::valid();
        }

        let mut blocking_hazard_ids = Vec::new();

        for hazard in drone.local_hazard_map.hazards() {
            if hazard.state != HazardState::Active {
                continue;
            }

            let blocks_route = remaining_segments
                .iter()
                .any(|segment| hazard.footprint.intersects_line_segment(segment));

            if blocks_route {
                blocking_hazard_ids.push(hazard.id.clone());
            }
        }

        // LocalHazardMap currently uses a HashMap, whose iteration order is not
        // deterministic. Sorting keeps validation results stable across runs.
        blocking_hazard_ids.sort_by(|left, right| {
            left.as_str().cmp(right.as_str())
        });

        RouteValidationResult::new(blocking_hazard_ids)
    }

    fn remaining_route_segments(drone: &SimDrone) -> Vec<LineSegment> {
        let Some(flight_plan_execution) = drone.flight_plan_execution.as_ref() else {
            return Vec::new();
        };

        let route_execution = &flight_plan_execution.route_execution;
        let route = &route_execution.route;
        let current_index = route_execution.current_waypoint_index;

        let Some(current_waypoint) = route.waypoints.get(current_index) else {
            return Vec::new();
        };

        let remaining_waypoints = &route.waypoints[current_index..];

        let mut segments = Vec::with_capacity(remaining_waypoints.len());

        // Validate the segment the drone is currently traversing.
        segments.push(LineSegment {
            start: drone.position,
            end: current_waypoint.position,
        });

        // Validate every later segment in the remaining route.
        segments.extend(
            remaining_waypoints
                .windows(2)
                .map(|waypoints| LineSegment {
                    start: waypoints[0].position,
                    end: waypoints[1].position,
                }),
        );

        segments
    }

    pub fn step(world: &mut SimulationWorld) {
        let mut events = Vec::new();

        for drone in world.drones_mut() {
            let result = Self::validate(drone);

            let Some(flight_plan_execution) = drone.flight_plan_execution.as_mut() else {
                continue;
            };

            let previous_status = flight_plan_execution.validation_status;

            let next_status = if result.is_valid() {
                ValidationStatus::Valid
            }
            else {
                ValidationStatus::Blocked
            };

            flight_plan_execution.validation_status = next_status;

            if previous_status == ValidationStatus::Valid
                && next_status == ValidationStatus::Blocked {

                let route_id = flight_plan_execution.route_execution.route.id.clone();

                for hazard_id in result.blocking_hazard_ids() {
                    events.push( SimulationEvent::RouteBlocked {
                        drone_id: drone.id.clone(),
                        route_id: route_id.clone(),
                        hazard_id: hazard_id.clone(),
                    });
                }
            }
        }

        for event in events {
            world.event_queue_mut().push(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{Circle, Point2},
        model::{
            FlightPlan, Hazard, HazardSeverity, HazardType, Route,
            SimDrone, Waypoint,
        },
    };

    fn drone_with_route() -> SimDrone {
        let route = Route::new(
            "route-001",
            vec![
                Waypoint::new("wp-001", Point2::new(10.0, 0.0)),
                Waypoint::new("wp-002", Point2::new(20.0, 0.0)),
            ],
        );

        let flight_plan =
            FlightPlan::new("fp-001", "mission-001", route);

        let mut drone =
            SimDrone::new("drone-001", Point2::new(0.0, 0.0), 10.0, 5.0);

        drone.assign_flight_plan(flight_plan);
        drone
    }

    fn active_hazard(
        id: &str,
        center: Point2,
        radius: f64,
    ) -> Hazard {
        Hazard::new(
            id.into(),
            Circle::new(center, radius),
            HazardType::StaticObstacle,
            HazardSeverity::High,
            HazardState::Active,
        )
    }

    #[test]
    fn route_without_known_hazards_is_valid() {
        let drone = drone_with_route();

        let result = RouteValidationSystem::validate(&drone);

        assert!(result.is_valid());
        assert!(result.blocking_hazard_ids().is_empty());
    }

    #[test]
    fn known_hazard_intersecting_current_segment_blocks_route() {
        let mut drone = drone_with_route();

        drone.local_hazard_map.insert(active_hazard(
            "hazard-001",
            Point2::new(5.0, 0.0),
            1.0,
        ));

        let result = RouteValidationSystem::validate(&drone);

        assert!(!result.is_valid());
        assert_eq!(
            result.blocking_hazard_ids(),
            &[HazardId::new("hazard-001")]
        );
    }

    #[test]
    fn known_hazard_away_from_route_does_not_block_route() {
        let mut drone = drone_with_route();

        drone.local_hazard_map.insert(active_hazard(
            "hazard-001",
            Point2::new(5.0, 10.0),
            1.0,
        ));

        let result = RouteValidationSystem::validate(&drone);

        assert!(result.is_valid());
    }

    #[test]
    fn cleared_hazard_does_not_block_route() {
        let mut drone = drone_with_route();

        let mut hazard = active_hazard(
            "hazard-001",
            Point2::new(5.0, 0.0),
            1.0,
        );
        hazard.state = HazardState::Cleared;

        drone.local_hazard_map.insert(hazard);

        let result = RouteValidationSystem::validate(&drone);

        assert!(result.is_valid());
    }

    #[test]
    fn valid_to_blocked_transition_emits_route_blocked_once() {
        let route = Route::new(
            "route-001",
            vec![
                Waypoint::new("wp-001", Point2::new(10.0, 0.0)),
                Waypoint::new("wp-002", Point2::new(20.0, 0.0)),
            ],
        );

        let flight_plan =
            FlightPlan::new("flight-plan-001", "mission-001", route);

        let mut drone =
            SimDrone::new("drone-001", Point2::new(0.0, 0.0), 10.0, 5.0);

        drone.assign_flight_plan(flight_plan);

        let hazard = Hazard::new(
            "hazard-001".into(),
            Circle::new(Point2::new(5.0, 0.0), 1.0),
            HazardType::StaticObstacle,
            HazardSeverity::High,
            HazardState::Active,
        );

        // Route validation uses the drone's local knowledge, not the
        // authoritative hazards stored in SimulationWorld.
        assert!(drone.local_hazard_map.insert(hazard.clone()));

        let mut world = SimulationWorld::new(
            vec![drone],
            vec![hazard],
        );

        RouteValidationSystem::step(&mut world);

        let drone = &world.drones()[0];
        let execution = drone
            .flight_plan_execution
            .as_ref()
            .expect("drone should have a flight plan execution");

        assert_eq!(
            execution.validation_status,
            ValidationStatus::Blocked
        );

        let events = world.drain_events();

        assert_eq!(events.len(), 1);

        assert_eq!(
            events[0],
            SimulationEvent::RouteBlocked {
                drone_id: DroneId::new("drone-001"),
                route_id: RouteId::new("route-001"),
                hazard_id: HazardId::new("hazard-001"),
            }
        );

        // Running validation again while the route remains blocked should
        // not emit a duplicate transition event.
        RouteValidationSystem::step(&mut world);

        assert!(world.drain_events().is_empty());
    }
}
