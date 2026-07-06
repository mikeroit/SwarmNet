use crate::math::Vector2;
use crate::model::{FlightPlan, FlightPlanExecution};

#[derive(Debug, Clone, PartialEq)]
pub struct SimDrone {
    pub id: String,
    pub position: Vector2,
    pub speed_mps: f64,
    pub flight_plan_execution: Option<FlightPlanExecution>
}

impl SimDrone {
    pub fn new(id: impl Into<String>, position: Vector2, speed_mps: f64) -> Self {
        Self {
            id: id.into(),
            position,
            speed_mps,
            flight_plan_execution: None,
        }
    }

    pub fn assign_flight_plan_execution(&mut self, flight_plan: FlightPlan) {
        self.flight_plan_execution = Some(FlightPlanExecution::new(flight_plan));
    }
}
