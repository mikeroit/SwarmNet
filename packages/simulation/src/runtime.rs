use crate::{Clock, Event, State, World};
use std::time::Duration;

#[derive(Debug)]
pub struct Runtime {
    clock: Clock,
    state: State,
    world: World,
    max_ticks: u64,
}

impl Runtime {
    pub fn new(tick_duration: Duration, max_ticks: u64, world: World) -> Self {
        Self {
            clock: Clock::new(tick_duration),
            state: State::Uninitialized,
            world,
            max_ticks,
        }
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn drain_events(&mut self) -> Vec<Event> {
        self.world.drain_events()
    }

    pub fn initialize(&mut self) {
        if self.state == State::Uninitialized {
            self.state = State::Initializing;
            self.state = State::Ready;
        }
    }

    pub fn start(&mut self) {
        if self.state == State::Ready {
            self.state = State::Running;
        }
    }

    pub fn tick(&mut self) {
        if self.state != State::Running {
            return;
        }

        self.clock.advance();
        self.world.update(self.clock.tick_duration());

        if self.clock.tick() >= self.max_ticks {
            self.state = State::Completed;
        }
    }

    pub fn shutdown(&mut self) {
        self.state = State::Shutdown;
    }
}

#[cfg(test)]
mod tests {
    use crate::MultiDroneScenario;

    use super::*;

    #[test]
    fn runtime_starts_uninitialized() {
        let runtime =
            Runtime::new(Duration::from_millis(100), 10, MultiDroneScenario::build());
        assert_eq!(runtime.state(), State::Uninitialized);
        assert_eq!(runtime.clock().tick(), 0);
    }

    #[test]
    fn runtime_initializes_to_ready() {
        let mut runtime =
            Runtime::new(Duration::from_millis(100), 10, MultiDroneScenario::build());
        runtime.initialize();

        assert_eq!(runtime.state(), State::Ready);
    }

    #[test]
    fn runtime_starts_running_after_ready() {
        let mut runtime =
            Runtime::new(Duration::from_millis(100), 10, MultiDroneScenario::build());
        runtime.initialize();
        runtime.start();

        assert_eq!(runtime.state(), State::Running);
    }

    #[test]
    fn runtime_completes_at_max_ticks() {
        let mut runtime =
            Runtime::new(Duration::from_millis(100), 3, MultiDroneScenario::build());
        runtime.initialize();
        runtime.start();

        runtime.tick();
        runtime.tick();
        runtime.tick();

        assert_eq!(runtime.clock().tick(), 3);
        assert_eq!(runtime.state(), State::Completed);
    }

    #[test]
    fn tick_does_nothing_unless_running() {
        let mut runtime =
            Runtime::new(Duration::from_millis(100), 3, MultiDroneScenario::build());
        runtime.tick();

        assert_eq!(runtime.clock().tick(), 0);
        assert_eq!(runtime.state(), State::Uninitialized);
    }
}
