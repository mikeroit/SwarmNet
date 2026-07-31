use simulation::{ConsoleRenderer, MultiDroneScenario, Runtime};
use std::time::Duration;

fn main() {
    let mut runtime = Runtime::new(Duration::from_millis(100), 30, MultiDroneScenario::build());

    println!("Initializing runtime...");
    runtime.initialize();

    println!("Starting simulation...");
    runtime.start();

    while runtime.state() == simulation::State::Running {
        runtime.tick();
        let events = runtime.drain_events();
        ConsoleRenderer::render(runtime.world(), &events, runtime.clock());
    }

    println!(" completed.");

    runtime.shutdown();

    println!("Runtime shutdown.");
}
