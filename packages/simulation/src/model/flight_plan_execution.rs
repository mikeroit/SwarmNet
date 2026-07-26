use crate::model::{FlightPlan, Route, RouteExecution, RouteId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    Pending,
    Active,
    Completed,
    Failed,
    Aborted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    Valid,
    Blocked,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlightPlanExecution {
    flight_plan: FlightPlan,
    pub route_execution: RouteExecution,
    pub execution_status: ExecutionStatus,
    pub validation_status: ValidationStatus,
    replan_count: u32,
}

impl FlightPlanExecution {
    pub fn new(flight_plan: FlightPlan) -> Self {
        let route = flight_plan.route().clone();

        Self {
            flight_plan,
            route_execution: RouteExecution::new(route),
            execution_status: ExecutionStatus::Pending,
            validation_status: ValidationStatus::Valid,
            replan_count: 0,
        }
    }

    pub fn flight_plan(&self) -> &FlightPlan {
        &self.flight_plan
    }

    pub fn replace_route(&mut self, new_route: Route) {
        self.route_execution = RouteExecution::new(new_route);
        self.validation_status = ValidationStatus::Valid;
    }

    pub fn next_replan_route_id(&mut self) -> RouteId {
        self.replan_count += 1;

        RouteId::from(format!(
            "{}-replan-{}",
            self.flight_plan.route().id(),
            self.replan_count,
        ))
    }
}
