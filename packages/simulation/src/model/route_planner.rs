#[cfg(test)]
use crate::math::{Circle, Point2};
#[cfg(test)]
use crate::model::{HazardSeverity, HazardState, HazardType};

use crate::math::{LineSegment, Vector2};
use crate::model::{Hazard, Route, RouteId, Waypoint};

const SAFETY_MARGIN_METERS: f64 = 1.0;

#[derive(Debug, Clone, PartialEq)]
pub struct RoutePlanner;

impl RoutePlanner {
    pub fn plan(
        route_id: impl Into<RouteId>,
        waypoints: &[Waypoint],
        hazards: &[&Hazard],
    ) -> Route {
        let hazards: Vec<&Hazard> = hazards.to_vec();
        let mut completed_route = Route::new(route_id, vec![]);

        for pair in waypoints.windows(2) {
            let segment = Self::plan_segment(&pair[0], &pair[1], &hazards);

            completed_route.extend(Route::new("", segment));
        }

        completed_route
    }

    pub fn plan_segment(start: &Waypoint, end: &Waypoint, hazards: &[&Hazard]) -> Vec<Waypoint> {
        let direct_segment = LineSegment::new(start.position, end.position);

        let mut blocking_hazards: Vec<&Hazard> = hazards
            .iter()
            .copied()
            .filter(|hazard| hazard.footprint.intersects_line_segment(&direct_segment))
            .collect();

        if blocking_hazards.is_empty() {
            return vec![start.clone(), end.clone()];
        }

        // Hash maps and scenario construction may not preserve a useful order.
        // Process hazards from nearest to farthest along the requested leg.
        blocking_hazards.sort_by(|left, right| {
            let left_distance = start.position.distance_to(&left.footprint.center);
            let right_distance = start.position.distance_to(&right.footprint.center);

            left_distance.total_cmp(&right_distance)
        });

        let route_vector = end.position - start.position;

        if route_vector.length_squared() == 0.0 {
            return vec![start.clone()];
        }

        let direction = route_vector.normalized();
        let normal = Vector2::new(-direction.y, direction.x);

        let mut result = Vec::with_capacity(2 + blocking_hazards.len() * 2);
        result.push(start.clone());

        for hazard in blocking_hazards {
            let clearance = hazard.footprint.radius + SAFETY_MARGIN_METERS;

            /*
             * Generate two deterministic points on one side of the hazard:
             *
             *                  before -------- after
             *                     \            /
             *   start ------------- (hazard) ------------- end
             *
             * Two points are preferable to a single point because they keep
             * the path outside the inflated hazard for more of the detour.
             */
            let before_position =
                hazard.footprint.center - direction * clearance + normal * clearance;

            let after_position =
                hazard.footprint.center + direction * clearance + normal * clearance;

            result.push(Waypoint::new(
                format!("replan-{}-before", hazard.id),
                before_position,
            ));

            result.push(Waypoint::new(
                format!("replan-{}-after", hazard.id),
                after_position,
            ));
        }

        result.push(end.clone());
        result
    }
}

#[test]
fn planned_segment_avoids_single_blocking_hazard() {
    let start = Waypoint::new("start", Point2::new(0.0, 0.0));
    let end = Waypoint::new("end", Point2::new(20.0, 0.0));

    let hazard = Hazard::new(
        "hazard-001".into(),
        Circle::new(Point2::new(10.0, 0.0), 2.0),
        HazardType::StaticObstacle,
        HazardSeverity::High,
        HazardState::Active,
    );

    let planned = RoutePlanner::plan_segment(&start, &end, &[&hazard]);

    assert!(planned.len() > 2);

    for pair in planned.windows(2) {
        let segment = LineSegment::new(pair[0].position, pair[1].position);

        assert!(
            !hazard.footprint.intersects_line_segment(&segment),
            "planned segment still intersects the hazard"
        );
    }
}

#[test]
fn replanned_route_from_current_drone_position_avoids_scenario_hazard() {
    let current_position = Waypoint::new(
        "replan-start-drone-001",
        Point2::new(0.8, 0.0),
    );

    let destination = Waypoint::new(
        "wp-002",
        Point2::new(10.0, 10.0),
    );

    let hazard = Hazard::new(
        "hazard-001".into(),
        Circle::new(Point2::new(2.0, 0.0), 1.0),
        HazardType::StaticObstacle,
        HazardSeverity::Low,
        HazardState::Active,
    );

    let planned_route = RoutePlanner::plan(
        "route-001",
        &[current_position, destination],
        &[&hazard],
    );

    assert!(
        planned_route.waypoints().len() > 2,
        "planner should insert detour waypoints for the blocking hazard"
    );

    for pair in planned_route.waypoints().windows(2) {
        let segment = LineSegment::new(
            pair[0].position,
            pair[1].position,
        );

        assert!(
            !hazard
                .footprint
                .intersects_line_segment(&segment),
            concat!(
                "replanned segment still intersects hazard: ",
                "{:?} -> {:?}"
            ),
            pair[0].position,
            pair[1].position,
        );
    }
}
