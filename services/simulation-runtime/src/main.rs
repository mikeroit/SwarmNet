use std::time::Duration;

use simulation::SimulationRuntime;

fn main() {
    let mut runtime = SimulationRuntime::new(
        Duration::from_millis(100),
        30,
    );

    println!("Initializing runtime...");
    runtime.initialize();

    println!("Starting simulation...");
    runtime.start();

    while runtime.state() == simulation::SimulationState::Running {
        runtime.tick();

        for drone in runtime.world().drones() {
            let route_status = drone
                .route
                .as_ref()
                .map(|route| {
                    if route.completed {
                        "completed".to_string()
                    } else {
                        format!("target wp index {}", route.current_waypoint_index)
                    }
                })
                .unwrap_or_else(|| "no route".to_string());

            println!(
                "Tick {:03} | Drone {} | Position: ({:.2}, {:.2}) | Route: {}",
                runtime.clock().tick(),
                drone.id,
                drone.position.x,
                drone.position.y,
                route_status,
            );
        }
    }

    println!("Simulation completed.");

    runtime.shutdown();

    println!("Runtime shutdown.");
}
