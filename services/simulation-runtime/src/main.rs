use std::time::Duration;

use simulation::SimulationRuntime;

fn main() {
    let mut runtime = SimulationRuntime::new(Duration::from_millis(100), 30);

    println!("Initializing runtime...");
    runtime.initialize();

    println!("Starting simulation...");
    runtime.start();

    while runtime.state() == simulation::SimulationState::Running {
        runtime.tick();
    }

    println!("Simulation completed.");

    runtime.shutdown();

    println!("Runtime shutdown.");
}
