use std::collections::HashSet;

use crate::model::HazardId;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocalHazardMap {
    detected_hazard_ids: HashSet<HazardId>,
}

impl LocalHazardMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_detected(&self, hazard_id: &HazardId) -> bool {
        self.detected_hazard_ids.contains(hazard_id)
    }

    pub fn mark_detected(&mut self, hazard_id: HazardId) -> bool {
        self.detected_hazard_ids.insert(hazard_id)
    }
}
