use crate::math::Point2;
use crate::model::{DroneId, FlightPlan, FlightPlanExecution};

#[derive(Debug, Clone, PartialEq)]
pub struct SimDrone {
    pub id: DroneId,
    pub position: Point2,
    pub speed_mps: f64,
    pub flight_plan_execution: Option<FlightPlanExecution>,
}

impl SimDrone {
    pub fn new(id: impl Into<DroneId>, position: Point2, speed_mps: f64) -> Self {
        Self {
            id: id.into(),
            position,
            speed_mps,
            flight_plan_execution: None,
        }
    }

    pub fn assign_flight_plan(&mut self, flight_plan: FlightPlan) {
        self.flight_plan_execution = Some(FlightPlanExecution::new(flight_plan));
    }
}
