use crate::model::{FlightPlan, RouteExecution};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    Pending,
    Active,
    Completed,
    Failed,
    Aborted,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlightPlanExecution {
    pub flight_plan: FlightPlan,
    pub route_execution: RouteExecution,
    pub status: ExecutionStatus,
}

impl FlightPlanExecution {
    pub fn new(flight_plan: FlightPlan) -> Self {
        let route = flight_plan.route.clone();

        Self {
            flight_plan,
            route_execution: RouteExecution::new(route),
            status: ExecutionStatus::Pending,
        }
    }
}
