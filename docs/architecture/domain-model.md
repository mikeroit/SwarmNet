# SwarmNet Domain Model

**Status:** Draft
**Last Updated:** 2026-07-02

---

# Purpose

This document defines the core domain concepts used throughout SwarmNet.

These definitions describe the meaning and responsibilities of each concept independent of any implementation language, database schema, or communication protocol.

The Protobuf contracts, Rust structs, database tables, and API models should all reflect the concepts defined here.

---

# Domain Philosophy

SwarmNet separates:

- Mission intent
- Autonomous execution
- Environmental awareness
- Vehicle state

Each concept has a single owner responsible for maintaining its authoritative state.

---

# Mission

## Definition

A Mission represents the strategic objective assigned by an operator.

A Mission describes **what** should be accomplished, not **how** each drone accomplishes it.

Examples include:

- Search an area
- Survey terrain
- Inspect infrastructure
- Deliver cargo
- Establish communications relay

## Responsibilities

A Mission:

- defines operational objectives
- defines mission constraints
- defines participating drones
- defines completion criteria
- owns one or more Flight Plans

## Owner

Mission Planner

---

# Flight Plan

## Definition

A Flight Plan is the executable assignment for a single drone.

It translates mission intent into a concrete sequence of actions for one vehicle.

Every drone participating in a mission receives its own Flight Plan.

## Responsibilities

A Flight Plan specifies:

- assigned route
- mission objectives
- operational constraints
- timing constraints
- contingency behavior

## Owner

Mission Planner

Execution responsibility belongs to the Drone Agent Runtime.

---
## FlightPlanExecution

A FlightPlanExecution tracks one drone's execution of a FlightPlan.

It owns:

- execution status
- route execution state
- progress toward completion

The FlightPlan itself remains a static assignment definition.
---

# Drone

## Definition

A Drone is an autonomous aerial vehicle capable of executing a Flight Plan.

The Drone Agent Runtime maintains the authoritative operational state of the drone.

## Responsibilities

A Drone:

- executes its Flight Plan
- maintains local state
- performs local route planning
- replans when conditions change
- detects hazards
- shares situational awareness
- publishes telemetry
- avoids conflicts with nearby drones

## Owner

Drone Agent Runtime

---

# Route

## Definition

A Route is a navigable path through space.

Routes describe **where** a drone intends to travel.

A Route is an immutable path definition. Changes to the active route are represented by creating a new Route and updating the associated RouteExecution.

## Responsibilities

A Route contains:

- ordered waypoints
- estimated distance
- estimated duration
- planning metadata

Routes do not contain mission objectives.

## Owner

Route Planner

---

## RouteExecution

A RouteExecution tracks one drone's progress through a Route.

It owns:

- current waypoint index
- completion state
- route execution progress

The Route itself remains a static path definition.
---

# Waypoint

## Definition

A Waypoint represents a navigational objective within a Route.

Waypoints describe positions the drone should reach or actions it should perform.

Examples include:

- travel location
- observation point
- loiter point
- inspection point
- landing point

## Owner

Route Planner

---

# Hazard

## Definition

A Hazard is any environmental condition that may require autonomous behavior to change.

Hazards are immutable descriptions of the environment.

Hazards may originate from onboard sensors, external systems, simulation, or operator input.

A Hazard represents a fact about the environment. It does not track which drones have detected or reacted to it.

Detection, reporting, sharing, and avoidance are modeled separately.

## Types

Examples include:

- terrain obstacle
- building
- weather
- restricted airspace
- GPS degradation
- communications loss
- another aircraft
- another drone

## Properties

Hazards may be:

- static
- dynamic
- temporary
- permanent
- local
- shared

## Owner

Hazard Processor

---

# Swarm

## Definition

A Swarm is a coordinated collection of drones participating in a common Mission.

The Swarm is a logical entity rather than a physical one.

## Responsibilities

The Swarm coordinates:

- shared situational awareness
- conflict avoidance
- cooperative behavior
- mission progress
- distributed decision making

## Owner

Swarm Coordinator

---

# Telemetry

## Definition

Telemetry is the stream of operational state published by each Drone Agent Runtime.

Telemetry provides situational awareness to the rest of the system.

## Examples

Telemetry may include:

- position
- velocity
- heading
- battery
- route progress
- mission status
- health
- communications quality

## Owner

Drone Agent Runtime

---

# Ownership Summary

| Domain Object | Authoritative Owner |
|---------------|---------------------|
| Mission | Mission Planner |
| Flight Plan | Mission Planner |
| Drone State | Drone Agent Runtime |
| Route | Route Planner |
| Waypoint | Route Planner |
| Hazard | Hazard Processor |
| Swarm State | Swarm Coordinator |
| Telemetry | Drone Agent Runtime |

---

# Relationship Diagram

```text
Mission
│
├── Flight Plan (Drone A)
│     │
│     ├── Route
│     │      └── Waypoints
│     │
│     └── Drone
│
├── Flight Plan (Drone B)
│     │
│     ├── Route
│     └── Drone
│
└── Flight Plan (Drone C)
      │
      ├── Route
      └── Drone

Hazards
     │
     ▼
Drone Agent Runtime
     │
     ▼
Route Replanner

Swarm
     │
     ▼
Swarm Coordinator
```

---
## Definition vs Execution Models

SwarmNet separates static definitions from runtime execution state.

Definitions describe what should happen:

- Mission
- FlightPlan
- Route
- Waypoint

Execution models describe what is happening now:

- FlightPlanExecution
- RouteExecution
- Drone runtime state
- SimulationClock

This separation prevents mutable execution state from contaminating reusable mission definitions.
---

# Guiding Principles

1. Every domain object has exactly one authoritative owner.

2. Mission intent is separate from autonomous execution.

3. Local autonomy should continue during temporary communication loss.

4. Domain objects are independent of transport technology.

5. Services communicate using Commands, Events, and Queries rather than direct object mutation.

6. All autonomy decisions should be explainable through recorded events.

7. Components should remain modular and replaceable in accordance with Open Arsenal / MOSA principles.
