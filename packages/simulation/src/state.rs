#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationState {
    Uninitialized,
    Initializing,
    Ready,
    Running,
    Completed,
    Failed,
    Shutdown,
}
