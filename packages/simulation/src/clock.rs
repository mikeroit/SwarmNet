use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationClock {
    tick: u64,
    tick_duration: Duration,
}

impl SimulationClock {
    pub fn new(tick_duration: Duration) -> Self {
        Self {
            tick: 0,
            tick_duration,
        }
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn tick_duration(&self) -> Duration {
        self.tick_duration
    }

    pub fn elapsed_time(&self) -> Duration {
        self.tick_duration * self.tick as u32
    }

    pub fn advance(&mut self) {
        self.tick += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_tick_zero() {
        let clock = SimulationClock::new(Duration::from_millis(100));
        assert_eq!(clock.tick(), 0);
        assert_eq!(clock.elapsed_time(), Duration::from_millis(0));
    }

    #[test]
    fn advances_one_tick() {
        let mut clock = SimulationClock::new(Duration::from_millis(100));
        clock.advance();

        assert_eq!(clock.tick(), 1);
        assert_eq!(clock.elapsed_time(), Duration::from_millis(100));
    }

    #[test]
    fn advances_multiple_ticks() {
        let mut clock = SimulationClock::new(Duration::from_millis(100));

        for _ in 0..10 {
            clock.advance();
        }

        assert_eq!(clock.tick(), 10);
        assert_eq!(clock.elapsed_time(), Duration::from_millis(1000));
    }
}
