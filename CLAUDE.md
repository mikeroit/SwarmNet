# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

SwarmNet is a distributed autonomous swarm coordination platform demonstrating modular mission-system architecture for autonomous UAS (drones). It is developed incrementally: every milestone should produce a working, executable system rather than a large speculative implementation.

Priority order for design/implementation decisions: **Correctness > Simplicity > Determinism > Modularity > Performance.**

**Longer-term intent:** the project is evolving toward an Open Arsenal-compliant mission system over NATS-based distributed messaging, and doubles as a portfolio piece demonstrating modern distributed-systems/autonomy architecture for defense-industry work. This shapes some priorities beyond pure engineering merit — e.g. architectural correctness and clean service boundaries are weighted more heavily than pathfinding sophistication in the near term (see Roadmap below), and professional-grade documentation/demo scenarios are an explicit deliverable, not just nice-to-haves.

### Requirements & constraints
- Must remain able to evolve into a distributed, decentralized system (not just single-process) without large rewrites.
- Hazard detection, publication, and cooperative avoidance must stay first-class capabilities, not bolted on later.
- Favor incremental evolution over large architectural rewrites at every stage.
- Should remain suitable as a portfolio-quality demonstration of defense-relevant software architecture (this affects documentation/demo priorities, not just code).

## Commands

This is a Cargo workspace (members: `packages/simulation`, `packages/swarmnet-proto`, `services/simulation-runtime`). A `pnpm-workspace.yaml` exists for future TypeScript packages under `apps/*` and `services/*`, but no TS/JS code exists yet.

```bash
# Build everything
cargo build --workspace

# Run all tests
cargo test --workspace

# Run tests for a single crate
cargo test -p simulation
cargo test -p swarmnet-proto

# Run a single test by name
cargo test -p simulation route_validation::tests::known_hazard_intersecting_current_segment_blocks_route

# Run the simulation executable (prints tick-by-tick console output)
cargo run -p simulation-runtime

# Start local infra (NATS + Postgres) — not yet wired into any Rust code
docker-compose up
```

There is no separate lint command configured beyond standard `cargo build`/`cargo check` warnings; there's no `clippy` or `rustfmt` CI step defined in-repo, but running `cargo fmt` / `cargo clippy` before finishing changes is reasonable practice.

## Repository layout

- `packages/simulation` — the deterministic simulation engine (all current logic lives here).
- `packages/swarmnet-proto` — Rust bindings generated from `proto/swarmnet/v1/*.proto` via `prost-build` (see `build.rs`). Regenerated automatically on build; do not hand-edit generated code. **Not yet consumed** by `packages/simulation` — the simulation currently uses its own plain Rust structs, not the protobuf types.
- `services/simulation-runtime` — thin binary crate (`main.rs`) that wires together a scenario, a `Runtime`, and the console renderer into an executable.
- `proto/swarmnet/v1/` — canonical Protobuf message contracts (common, drone, mission, hazard, route, telemetry, flight_plan).
- `docs/architecture/` — authoritative design docs (see below); read these before making structural changes.
- `docs/decisions/` — numbered ADRs explaining *why* key choices were made.

## Architecture

SwarmNet uses a **hybrid autonomy architecture** (ADR 0001): a ground station owns mission intent (mission definition, scenario management, initial route intent) while each drone agent owns local execution (local route planning, hazard processing, telemetry, short-horizon conflict avoidance). Full component/deployment breakdown is in `docs/architecture/system-architecture.md`. Only the Rust simulation side is implemented today; the TypeScript ground-station side (Command & Control API, Operator Dashboard, Mission Planner, Event Log Service) described in the docs does not exist yet.

### Definition vs. execution state (ADR 0006)

A recurring pattern throughout the model layer: static definitions are separated from mutable runtime state. Always extend the pair together, don't collapse them:

- `Route` (immutable path/waypoints) vs. `RouteExecution` (current waypoint index, progress) — `packages/simulation/src/model/route.rs` / `route_execution.rs`
- `FlightPlan` (static assignment) vs. `FlightPlanExecution` (execution/validation status, replan count) — `flight_plan.rs` / `flight_plan_execution.rs`
- `Hazard` is an immutable fact about the environment; detection/sharing/reaction to it are modeled as separate systems and per-drone `LocalHazardMap`, not fields on `Hazard` itself.

Changing an active route means constructing a *new* `Route` and calling `FlightPlanExecution::replace_route`/`next_replan_route_id` — never mutate a `Route` in place.

### Simulation tick pipeline

`World::update()` (`packages/simulation/src/model/world.rs`) is the fixed-timestep heart of the simulation and runs a strict, ordered pipeline every tick — this order is deliberate and documented in `docs/architecture/simulation-architecture.md`:

1. `RouteFollowingSystem` — move each drone along its route
2. `HazardDetectionSystem` — each drone detects nearby hazards, emitting `HazardObservation`s
3. Observations are published to a `HazardObservationTransport` (currently `InProcessHazardObservationTransport`; the trait boundary exists so a real transport, e.g. NATS, can be substituted later) and drained back out
4. `HazardSharingSystem` — merges received observations into every drone's `LocalHazardMap`
5. `RouteValidationSystem` — checks whether each drone's current route is now blocked by a known hazard
6. `RoutePlanningSystem` — replans blocked routes

Systems live in `packages/simulation/src/systems/`, are stateless structs with a `step(...)` associated function, and operate on `&mut World`. When adding new tick behavior, add a new system and slot it into this pipeline in `World::update` rather than embedding logic elsewhere.

The `Runtime` (`packages/simulation/src/runtime.rs`) wraps `World` with a `Clock` and an explicit lifecycle `State` (`Uninitialized → Initializing → Ready → Running → Completed/Failed → Shutdown`, see `state.rs`). The full intended state machine — including `Paused`/`Stepping` for operator/dashboard control — is documented in `docs/architecture/simulation-runtime-lifecycle.md`; only the MVP subset is implemented. `tick()` is a no-op unless `state == Running`.

### Domain events

Systems emit `DomainEvent`s onto a `World`-owned `EventQueue` (`packages/simulation/src/events/`); `Runtime::drain_events()` flushes them once per tick for consumers (currently `ConsoleRenderer`). Events are the intended audit trail for autonomy decisions per the domain model's guiding principles — prefer emitting an event over a bare state mutation when something notable happens (hazard detected, route blocked, route replanned, etc.).

### IDs

All domain IDs (`DroneId`, `RouteId`, `HazardId`, `WaypointId`, `FlightPlanId`, `MissionId`) are newtype wrappers generated by the `define_id!` macro in `packages/simulation/src/model/ids.rs`, constructed via `.into()` from `&str`/`String`. Several have a `display_name()` used by the console renderer.

### Coordinates and units (`docs/architecture/simulation-conventions.md`)

- 2D Cartesian, right-handed: +X = East, +Y = North (no altitude/3D yet, though the Protobuf model supports geographic coordinates for later use).
- Distances in meters, velocity in meters/second, time in simulation seconds, angles in radians.

### Scenarios

`packages/simulation/src/scenarios/` builds `World` instances for known starting configurations (`SimpleScenario` = one drone, `MultiDroneScenario` = three drones plus a hazard). Use these as the pattern for adding new test/demo scenarios rather than constructing `World` ad hoc.

## Current state (as of last handoff)

**Built:** immutable mission model, separate execution-state model, systems-over-world simulation architecture, world update loop + runtime, basic route execution, hazard model, basic route replanning. Just came off a major refactor separating planning from execution.

**In progress:** transitioning from that refactor back into feature development; preparing the architecture for distributed-messaging integration (NATS not yet wired in — see Commands section, infra exists but isn't consumed by any Rust code yet).

**Known technical debt:**
- Path planner is intentionally temporary/simplistic — architecture was prioritized over algorithm quality early on.
- Replanning edge-case handling is limited.
- Distributed networking layer doesn't exist yet.
- Some runtime/time-ownership cleanup was recently completed but may not be fully settled.

## Roadmap (priority order)

1. **Finish core single-process simulation** — remaining execution systems, mission execution flow, keep validating architecture before distributing.
2. **Expand hazard system** — more hazard types, better lifecycle management, refined detection/publication.
3. **Replace temporary pathfinding** — more capable routing/replanning, dynamic in-flight avoidance.
4. **Cooperative swarm behavior** — hazard sharing between drones, coordinated route updates, conflict-free replanning.
5. **Freeze Protocol Buffers as the transport-independent contract** (schemas already exist in `proto/`; this milestone is about consuming them from `packages/simulation`, not defining new ones).
6. **Integrate NATS** — replace in-process transport with real pub/sub, validate message-driven behavior.
7. **Split into distributed microservices** along domain boundaries.
8. **Distributed drone nodes** — each drone executes independently, world state synced via messaging.
9. **Open Arsenal / OMS alignment** for external interoperability.
10. Later: simulation fidelity (flight dynamics, sensing realism), visualization/operator tooling, performance/scale work, CI/CD + deployment, docs, and portfolio polish (demo scenarios, metrics, interview-ready writeups).

## Open questions / not yet decided

- Final pathfinding algorithm choice.
- Exact service boundaries once distribution begins.
- How much world state gets replicated vs. queried through messaging.
- Conflict-resolution strategy when drones publish overlapping/conflicting hazard info.
- Long-term synchronization model for distributed simulation time.
- How far to take Open Arsenal compatibility beyond core messaging.

## Conventions worth knowing

- `derive-new` (`#[derive(new)]`) is used to generate constructors where it reduces boilerplate — see `World::new` in `world.rs` for its `#[new(value = "...")]` field-default syntax.
- Domain objects have exactly one authoritative owner (documented per-concept in `docs/architecture/domain-model.md`); when adding a field, check which owner's module it actually belongs in rather than bolting it onto whichever struct is convenient.
- Keep model/domain code independent of any specific transport — `HazardObservationTransport` is the pattern to follow (trait in `messaging/`, in-process impl for tests/sim, real impl added later without touching call sites).
