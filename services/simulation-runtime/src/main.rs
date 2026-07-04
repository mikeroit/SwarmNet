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

        for drone in runtime.world().drones() {
            println!(
            "Tick {:03} | Drone {} | Position: ({:.2}, {:.2})",
            runtime.clock().tick(),
            drone.id,
            drone.position.x,
            drone.position.y,
            );
        }
    }

    println!("Simulation completed.");

    runtime.shutdown();

    println!("Runtime shutdown.");
}
