use crate::events::SimulationEvent;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EventQueue {
    events: Vec<SimulationEvent>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: SimulationEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[SimulationEvent] {
        &self.events.as_slice()
    }

    pub fn drain(&mut self) -> Vec<SimulationEvent> {
        self.events.drain(..).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}
