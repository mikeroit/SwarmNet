# ADR 0001: Use Hybrid Autonomy Architecture

## Status

Accepted

## Context

SwarmNet needs to demonstrate autonomous drone coordination while remaining understandable, testable, and suitable for incremental development.

A fully centralized architecture would be simpler, but it would not demonstrate resilient drone-local autonomy. A fully distributed architecture would demonstrate autonomy, but it would add substantial complexity early in the project.

## Decision

SwarmNet will use a hybrid autonomy architecture.

The ground station is responsible for:

- Mission definition
- Operator control
- Scenario management
- Initial mission intent
- Event logging
- Long-term state persistence

Each Drone Agent Runtime is responsible for:

- Mission execution
- Local state management
- Local route planning
- Dynamic replanning
- Hazard processing
- Peer awareness
- Short-horizon conflict avoidance
- Telemetry publication

## Consequences

Benefits:

- Clear separation between mission intent and autonomous execution
- Better resilience during communication loss
- Stronger demonstration of distributed systems and edge autonomy
- Easier incremental development than a fully distributed architecture
- Good alignment with modular open-system design principles

Tradeoffs:

- More complex than a centralized planner
- Drone agents must maintain local world state
- Route decisions may differ between agents if their hazard maps diverge
- Requires careful event logging to explain autonomous decisions

## Compliance Notes

This decision supports a modular architecture where mission planning, route planning, hazard processing, and drone execution can evolve as separate replaceable components.
