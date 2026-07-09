# Simulation Conventions

## Coordinate System

SwarmNet uses a right-handed 2D Cartesian coordinate system for the MVP simulation.

- +X = East
- +Y = North

Future versions may extend this to full 3D coordinates.

---

## Units

All distances are expressed in meters.

Examples:

- Drone position
- Route waypoint positions
- Hazard radius

Velocity is expressed in meters per second.

Time is expressed in simulation seconds.

Angles are expressed in radians unless explicitly documented otherwise.

---

## Simulation Tick

Simulation advances using a fixed timestep.

All systems execute once per simulation tick.

The simulation is deterministic given identical initial conditions and event ordering.
