use crate::{
    SimulationClock, SimulationEvent, SimulationState, SimulationWorld,
};
use std::time::Duration;

#[derive(Debug)]
pub struct SimulationRuntime {
    clock: SimulationClock,
    state: SimulationState,
    world: SimulationWorld,
    max_ticks: u64,
}

impl SimulationRuntime {
    pub fn new(tick_duration: Duration, max_ticks: u64, world: SimulationWorld) -> Self {
        Self {
            clock: SimulationClock::new(tick_duration),
            state: SimulationState::Uninitialized,
            world,
            max_ticks,
        }
    }

    pub fn clock(&self) -> &SimulationClock {
        &self.clock
    }

    pub fn state(&self) -> SimulationState {
        self.state
    }

    pub fn world(&self) -> &SimulationWorld {
        &self.world
    }

    pub fn drain_events(&mut self) -> Vec<SimulationEvent> {
        self.world.drain_events()
    }

    pub fn initialize(&mut self) {
        if self.state == SimulationState::Uninitialized {
            self.state = SimulationState::Initializing;
            self.state = SimulationState::Ready;
        }
    }

    pub fn start(&mut self) {
        if self.state == SimulationState::Ready {
            self.state = SimulationState::Running;
        }
    }

    pub fn tick(&mut self) {
        if self.state != SimulationState::Running {
            return;
        }

        self.clock.advance();
        self.world.update(self.clock.tick_duration());

        if self.clock.tick() >= self.max_ticks {
            self.state = SimulationState::Completed;
        }
    }

    pub fn shutdown(&mut self) {
        self.state = SimulationState::Shutdown;
    }
}

#[cfg(test)]
mod tests {
    use crate::MultiDroneScenario;

    use super::*;

    #[test]
    fn runtime_starts_uninitialized() {
        let runtime = SimulationRuntime::new(Duration::from_millis(100), 10, MultiDroneScenario::build());
        assert_eq!(runtime.state(), SimulationState::Uninitialized);
        assert_eq!(runtime.clock().tick(), 0);
    }

    #[test]
    fn runtime_initializes_to_ready() {
        let mut runtime = SimulationRuntime::new(Duration::from_millis(100), 10, MultiDroneScenario::build());
        runtime.initialize();

        assert_eq!(runtime.state(), SimulationState::Ready);
    }

    #[test]
    fn runtime_starts_running_after_ready() {
        let mut runtime = SimulationRuntime::new(Duration::from_millis(100), 10, MultiDroneScenario::build());
        runtime.initialize();
        runtime.start();

        assert_eq!(runtime.state(), SimulationState::Running);
    }

    #[test]
    fn runtime_completes_at_max_ticks() {
        let mut runtime = SimulationRuntime::new(Duration::from_millis(100), 3, MultiDroneScenario::build());
        runtime.initialize();
        runtime.start();

        runtime.tick();
        runtime.tick();
        runtime.tick();

        assert_eq!(runtime.clock().tick(), 3);
        assert_eq!(runtime.state(), SimulationState::Completed);
    }

    #[test]
    fn tick_does_nothing_unless_running() {
        let mut runtime = SimulationRuntime::new(Duration::from_millis(100), 3, MultiDroneScenario::build());
        runtime.tick();

        assert_eq!(runtime.clock().tick(), 0);
        assert_eq!(runtime.state(), SimulationState::Uninitialized);
    }
}
