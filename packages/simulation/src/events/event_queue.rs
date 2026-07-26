use crate::events::Event;
use derive_new;


#[derive(Debug, Default, Clone, PartialEq, Eq, derive_new::new)]
pub struct EventQueue {
    events: Vec<Event>,
}

impl EventQueue {
    //pub fn new() -> Self {
        //Self { events: Vec::new() }
    //}

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[Event] {
        self.events.as_slice()
    }

    pub fn drain(&mut self) -> Vec<Event> {
        self.events.drain(..).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}
