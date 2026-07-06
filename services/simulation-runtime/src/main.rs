use std::time::Duration;

use simulation::{SimulationRuntime};

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
                .flight_plan_execution
                .as_ref()
                .map(|flight_plan_execution| {
                    if flight_plan_execution.route_execution.completed {
                        "completed".to_string()
                    }
                    else {
                        format!("target wp index {}", flight_plan_execution.route_execution.current_waypoint_index)
                    }

                }).expect(format!("expected to find flight plan execution for drone: {}", drone.id).as_str());


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
