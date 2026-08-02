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
    route_execution: RouteExecution,
    execution_status: ExecutionStatus,
    validation_status: ValidationStatus,
    replan_count: u32,
}

impl FlightPlanExecution {
    pub fn flight_plan(&self) -> &FlightPlan {
        &self.flight_plan
    }

    pub fn replace_route(&mut self, new_route: Route) {
        self.route_execution = RouteExecution::from(new_route);
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

    pub fn route_execution(&self) -> &RouteExecution {
        &self.route_execution
    }

    pub fn route_execution_mut(&mut self) -> &mut RouteExecution {
        &mut self.route_execution
    }

    pub fn execution_status(&self) -> ExecutionStatus {
        self.execution_status
    }

    pub fn validation_status(&self) -> ValidationStatus {
        self.validation_status
    }

    pub fn mark_route_blocked(&mut self) {
        self.validation_status = ValidationStatus::Blocked;
    }

    pub fn mark_route_valid(&mut self) {
        self.validation_status = ValidationStatus::Valid;
    }

    pub fn start_execution(&mut self) {
        self.execution_status = ExecutionStatus::Active;
    }

    pub fn complete_execution(&mut self) {
        self.execution_status = ExecutionStatus::Completed;
    }
}

impl From<FlightPlan> for FlightPlanExecution {
    fn from(flight_plan: FlightPlan) -> Self {
        let route = flight_plan.route().clone();

        Self {
            flight_plan,
            route_execution: RouteExecution::from(route),
            execution_status: ExecutionStatus::Pending,
            validation_status: ValidationStatus::Valid,
            replan_count: 0,
        }
    }
}
