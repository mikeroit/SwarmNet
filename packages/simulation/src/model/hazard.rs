use crate::math::Point2;
use crate::model::HazardId;

#[derive(Debug, Clone, PartialEq)]
pub enum HazardType {
    StaticObstacle,
    NoFlyZone,
    Weather,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HazardSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HazardState {
    Active,
    Cleared,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hazard {
    pub id: HazardId,
    pub position: Point2,
    pub radius: f64,
    pub hazard_type: HazardType,
    pub severity: HazardSeverity,
    pub state: HazardState,
}

impl Hazard {
    pub fn new(
        id: HazardId,
        position: Point2,
        radius: f64,
        hazard_type: HazardType,
        severity: HazardSeverity,
        state: HazardState,
    ) -> Self {
        Self {
            id,
            position,
            radius,
            hazard_type,
            severity,
            state,
        }
    }
}
