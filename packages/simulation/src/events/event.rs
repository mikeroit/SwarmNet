use crate::model::{DroneId, FlightPlanId, RouteId, WaypointId};

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
}
