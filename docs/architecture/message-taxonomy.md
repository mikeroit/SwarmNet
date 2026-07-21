# SwarmNet Message Taxonomy

**Status:** Draft
**Last Updated:** 2026-07-02

## Purpose

This document defines how SwarmNet components communicate.

SwarmNet uses three primary message categories:

- Commands
- Events
- Queries

Domain objects define what entities exist in the system. Messages define how components interact.

## Commands

A Command requests that a component perform an action.

Commands are imperative and should usually have one intended handler.

Examples:

| Command | Intended Handler |
|---|---|
| CreateMission | Mission Planner |
| AssignFlightPlan | Drone Agent Runtime |
| RequestRoutePlan | Route Planner |
| RequestRouteReplan | Path Replanner |
| ReportHazard | Hazard Processor |
| AbortMission | Mission Planner / Drone Agent Runtime |

## Events

An Event records that something already happened.

Events are declarative and may have many subscribers.

Examples:

| Event | Publisher |
|---|---|
| MissionCreated | Mission Planner |
| FlightPlanAssigned | Mission Planner |
| DroneTelemetryPublished | Drone Agent Runtime |
| HazardDetected | Hazard Processor |
| RoutePlanned | Route Planner |
| RouteInvalidated | Path Replanner |
| RouteReplanned | Path Replanner |
| ConflictDetected | Swarm Coordinator |
| AvoidanceManeuverIssued | Swarm Coordinator |
| MissionCompleted | Mission Planner / Drone Agent Runtime |

## Queries

A Query requests current or historical state.

Queries may be synchronous through an API or asynchronous through request/reply messaging.

Examples:

| Query | Handler |
|---|---|
| GetMissionStatus | Mission Planner / Event Log Service |
| GetDroneState | Drone Agent Runtime / Event Log Service |
| GetHazardMap | Hazard Map Component |
| GetRoute | Route Planner / Event Log Service |
| GetEventHistory | Event Log Service |

## Naming Rules

Commands should use imperative names:

- CreateMission
- AssignFlightPlan
- RequestRoutePlan

Events should use past-tense names:

- MissionCreated
- FlightPlanAssigned
- HazardDetected

Queries should use retrieval names:

- GetMissionStatus
- GetDroneState
- GetEventHistory

## NATS Subject Naming

Initial NATS subjects should use this pattern:

```text
swarmnet.<version>.<category>.<entity>.<action>
