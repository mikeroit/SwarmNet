use crate::events::DomainEvent;
use derive_new;


#[derive(Debug, Default, Clone, PartialEq, Eq, derive_new::new)]
pub struct EventQueue {
    events: Vec<DomainEvent>,
}

impl EventQueue {
    //pub fn new() -> Self {
        //Self { events: Vec::new() }
    //}

    pub fn push(&mut self, event: DomainEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[DomainEvent] {
        self.events.as_slice()
    }

    pub fn drain(&mut self) -> Vec<DomainEvent> {
        self.events.drain(..).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}
