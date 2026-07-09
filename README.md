# SwarmNet

A distributed autonomous swarm coordination platform demonstrating modular mission-system architecture for autonomous UAS.

## Goals

- Distributed path planning
- Dynamic hazard sharing
- Autonomous route replanning
- Swarm deconfliction
- Event-driven architecture
- Open Arsenal / MOSA-inspired modular design

## Development Principles

SwarmNet is developed incrementally.

Every milestone should produce a working, executable system.

Priority order:

1. Correctness
2. Simplicity
3. Determinism
4. Modularity
5. Performance

Architecture evolves through small, testable iterations rather than large speculative implementations.

## Current Capabilities

- Fixed-timestep deterministic simulation
- Multiple autonomous drones
- Flight plan execution
- Route execution
- Waypoint following
- Simulation event queue
- Hazard modeling
- Console rendering
