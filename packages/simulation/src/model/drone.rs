use derive_new::new;

use crate::math::Point2;
use crate::model::{DroneId, FlightPlan, FlightPlanExecution, LocalHazardMap};

#[derive(Debug, Clone, PartialEq, new)]
pub struct SimDrone {
    pub id: DroneId,
    pub position: Point2,
    pub speed_mps: f64,
    pub sensor_range_meters: f64,
    #[new(default)]
    pub local_hazard_map: LocalHazardMap,
    #[new(value = "None")]
    pub flight_plan_execution: Option<FlightPlanExecution>,
}

impl SimDrone {
    pub fn assign_flight_plan(&mut self, flight_plan: FlightPlan) {
        self.flight_plan_execution = Some(FlightPlanExecution::from(flight_plan));
    }
}
