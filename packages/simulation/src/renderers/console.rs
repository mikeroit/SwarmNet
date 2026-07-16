use crate::SimDrone;
use crate::SimulationClock;
use crate::SimulationEvent;
use crate::SimulationWorld;

#[derive(Debug)]
pub struct ConsoleRenderer;

impl ConsoleRenderer {
    pub fn render(world: &SimulationWorld, events: &[SimulationEvent], clock: &SimulationClock) {
        print_header(clock);

        println!("");

        print_drone_states(world.drones());

        println!("");

        print_events(events);

        println!("");
    }
}

fn print_header(clock: &SimulationClock) {
    let header = format!(
        "Tick: {}      Sim Time: {:?}",
        clock.tick(),
        clock.elapsed_time()
    );

    println!("=========================================================");
    println!("{}", header);
    println!("=========================================================");
}

fn print_drone_states(drones: &[SimDrone]) {
    println!("DRONES");
    println!("");
    for drone in drones {
        print_drone_state(drone);
    }
}

fn print_drone_state(drone: &SimDrone) {
    println!("{}", drone.id.display_name());

    println!("  Position: ({}, {})", drone.position.x, drone.position.y);

    if let Some(flight_plan_execution) = drone.flight_plan_execution.as_ref() {
        println!("  Flight Plan: {}", flight_plan_execution.flight_plan.id);

        println!(
            "  Route: {}",
            flight_plan_execution.route_execution.route.id
        );

        if let Some(waypoint) = flight_plan_execution.route_execution.current_waypoint() {
            println!("  Current Waypoint: {}", waypoint.id);
        } else {
            println!("  Current Waypoint: None");
        }

        println!("  Status: {:?}", flight_plan_execution.execution_status);
    } else {
        println!("No Flight Plan Assigned");
    }
}

fn print_events(events: &[SimulationEvent]) {
    println!("EVENTS");
    println!("");

    for event in events {
        println!("{}", event);
    }
}
