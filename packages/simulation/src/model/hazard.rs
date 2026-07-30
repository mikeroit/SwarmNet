
use derive_new::new;

use crate::math::Circle;
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

#[derive(Debug, Clone, PartialEq, new)]
pub struct Hazard {
    pub id: HazardId,
    pub footprint: Circle,
    pub hazard_type: HazardType,
    pub severity: HazardSeverity,
    pub state: HazardState,
}

