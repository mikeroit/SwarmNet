use std::time::Duration;

use simulation::SimulationRuntime;

fn main() {
    let mut runtime = SimulationRuntime::new(
        Duration::from_millis(100),
        10,
    );

    println!("Initializing runtime...");
    runtime.initialize();

    println!("Starting simulation...");
    runtime.start();

    while runtime.state() == simulation::SimulationState::Running {
        runtime.tick();

        println!(
            "Tick {:03}  Sim Time: {:?}",
            runtime.clock().tick(),
            runtime.clock().elapsed_time(),
        );
    }

    println!("Simulation completed.");

    runtime.shutdown();

    println!("Runtime shutdown.");
}
