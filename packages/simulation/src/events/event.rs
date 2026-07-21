use crate::model::{DroneId, FlightPlanId, HazardId, RouteId, WaypointId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationEvent {
    WaypointReached {
        drone_id: DroneId,
        route_id: RouteId,
        waypoint_id: WaypointId,
    },
    RouteCompleted {
        drone_id: DroneId,
        route_id: RouteId,
    },
    FlightPlanCompleted {
        drone_id: DroneId,
        flight_plan_id: FlightPlanId,
    },
    HazardDetected {
        drone_id: DroneId,
        hazard_id: HazardId,
    },
    RouteBlocked {
        drone_id: DroneId,
        route_id: RouteId,
        hazard_id: HazardId,
    },
    RouteReplanned {
        drone_id: DroneId,
        route_id: RouteId,
    },
}

impl std::fmt::Display for SimulationEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationEvent::WaypointReached {
                drone_id,
                route_id: _,
                waypoint_id,
            } => {
                write!(
                    f,
                    "{} reached Waypoint {}",
                    drone_id.display_name(),
                    waypoint_id
                )
            }

            SimulationEvent::RouteCompleted { drone_id, route_id } => {
                write!(
                    f,
                    "{} completed Route {}",
                    drone_id.display_name(),
                    route_id
                )
            }

            SimulationEvent::FlightPlanCompleted {
                drone_id,
                flight_plan_id,
            } => {
                write!(
                    f,
                    "{} completed Flight Plan {}",
                    drone_id.display_name(),
                    flight_plan_id
                )
            }

            SimulationEvent::HazardDetected {
                drone_id,
                hazard_id,
            } => {
                write!(
                    f,
                    "{} detected Hazard: {}",
                    drone_id.display_name(),
                    hazard_id
                )
            }
            SimulationEvent::RouteBlocked {
                drone_id,
                route_id,
                hazard_id,
            } => {
                write!(
                    f,
                    "{} for {} blocked by {}",
                    route_id.display_name(),
                    drone_id.display_name(),
                    hazard_id.display_name(),
                )
            }
            SimulationEvent::RouteReplanned { drone_id, route_id } => {
                write!(
                    f,
                    "{} replanned {}",
                    drone_id.display_name(),
                    route_id.display_name(),
                )
            }
        }
    }
}
