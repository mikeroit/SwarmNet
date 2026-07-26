#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Uninitialized,
    Initializing,
    Ready,
    Running,
    Completed,
    Failed,
    Shutdown,
}
