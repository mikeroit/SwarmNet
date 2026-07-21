# SwarmNet System Architecture

## 1. Architectural Style

SwarmNet uses a hybrid autonomy architecture.

The ground station is responsible for mission definition, operator control, scenario management, and initial mission planning. Each drone agent is responsible for local execution, local route planning, hazard processing, telemetry publication, and short-horizon conflict avoidance.

This creates a separation between centralized mission intent and decentralized autonomous execution.

## 2. Primary Architecture Decision

SwarmNet does not use a purely centralized route-planning model.

Instead:

- The ground station defines mission objectives and initial route intent.
- Drone agents receive mission assignments.
- Drone agents maintain local state.
- Drone agents process hazards.
- Drone agents can replan locally when conditions change.
- The swarm coordination layer handles shared deconfliction rules and peer-state awareness.

## 3. Major Components

| Component | Runtime | Responsibility |
|---|---|---|
| Command & Control API | TypeScript | Mission control, operator commands, scenario control |
| Operator Dashboard | TypeScript / React | Visualization of drones, routes, hazards, telemetry, alerts |
| Mission Planner | TypeScript or Rust | Creates initial mission objectives and route intent |
| Drone Agent Runtime | Rust | On-drone autonomy runtime and local decision-making |
| Route Planner | Rust | Computes routes through known airspace constraints |
| Path Replanner | Rust | Updates routes when hazards or conflicts invalidate the current path |
| Hazard Processor | Rust | Converts hazard reports into local/shared hazard map updates |
| Swarm Coordinator | Rust | Detects and resolves drone-to-drone conflicts |
| Simulation Runtime | Rust | Simulates drones, hazards, movement, and communications |
| Event Log Service | TypeScript | Stores mission events, decisions, and telemetry snapshots |
| Message Bus | NATS | Asynchronous communication backbone |
| Database | PostgreSQL | Mission records, event logs, state snapshots, scenario data |

## 4. Deployment Zones

### Ground Station

Runs operator-facing and orchestration services:

- Command & Control API
- Operator Dashboard
- Mission Planner
- Event Log Service
- PostgreSQL
- NATS

### Drone / Edge Runtime

Runs autonomous execution services:

- Drone Agent Runtime
- Local Route Planner
- Local Path Replanner
- Local Hazard Processor
- Local Conflict Avoidance

### Simulation Environment

For MVP development, simulated drones run locally as software agents. Later, the same drone-agent runtime should be usable as an edge-deployable component.

## 5. Communication Model

SwarmNet uses asynchronous event-driven communication as the default.

NATS is the initial runtime message bus. Protobuf defines canonical message contracts.

Transport should remain replaceable. Message contracts should not depend on NATS-specific behavior.

## 6. Core Message Categories

### Commands

Commands request that a component perform an action.

Examples:

- CreateMission
- AssignMission
- RequestRoutePlan
- RequestRouteReplan
- ReportHazard
- AbortMission

### Events

Events describe something that already happened.

Examples:

- MissionCreated
- MissionAssigned
- DroneTelemetryPublished
- HazardDetected
- RoutePlanned
- RouteInvalidated
- RouteReplanned
- ConflictDetected
- AvoidanceManeuverIssued
- MissionCompleted

### Queries

Queries retrieve state.

Examples:

- GetDroneState
- GetMissionStatus
- GetHazardMap
- GetRoute
- GetEventHistory

## 7. High-Level System Flow

```mermaid
flowchart TD
    Operator[Operator Dashboard]
    API[Command & Control API]
    Mission[Mission Planner]
    Bus[NATS Message Bus]
    Drone[Drone Agent Runtime]
    Route[Local Route Planner]
    Replan[Path Replanner]
    Hazard[Hazard Processor]
    Swarm[Swarm Coordinator]
    EventLog[Event Log Service]
    DB[(PostgreSQL)]

    Operator --> API
    API --> Mission
    Mission --> Bus

    Bus --> Drone
    Drone --> Route
    Drone --> Replan
    Drone --> Hazard
    Drone --> Swarm

    Drone --> Bus
    Hazard --> Bus
    Swarm --> Bus
    Replan --> Bus

    Bus --> EventLog
    EventLog --> DB
    API --> DB
