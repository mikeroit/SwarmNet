use std::collections::HashMap;

use crate::model::{Hazard, HazardId};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LocalHazardMap {
    hazards: HashMap<HazardId, Hazard>,
}

impl LocalHazardMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains(&self, hazard_id: &HazardId) -> bool {
        self.hazards.contains_key(hazard_id)
    }

    /// Inserts or replaces a hazard.
    ///
    /// Returns `true` when the hazard ID was not previously known.
    /// Returns `false` when an existing hazard with the same ID was replaced.
    pub fn insert(&mut self, hazard: Hazard) -> bool {
        let hazard_id = hazard.id.clone();
        self.hazards.insert(hazard_id, hazard).is_none()
    }

    pub fn get(&self, hazard_id: &HazardId) -> Option<&Hazard> {
        self.hazards.get(hazard_id)
    }

    pub fn hazards(&self) -> impl Iterator<Item = &Hazard> {
        self.hazards.values()
    }

    pub fn len(&self) -> usize {
        self.hazards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hazards.is_empty()
    }
}
