#[cfg(test)]
use crate::math::{Circle, Point2};
#[cfg(test)]
use crate::model::{HazardType, HazardSeverity, HazardState};

use crate::math::{LineSegment, Vector2};
use crate::model::{Hazard, Route, Waypoint};

const SAFETY_MARGIN_METERS: f64 = 1.0;

#[derive(Debug, Clone, PartialEq)]
pub struct RoutePlanner;

impl RoutePlanner {
    pub fn plan(
        route_id: impl Into<crate::model::RouteId>,
        waypoints: &[Waypoint],
        hazards: &[Hazard],
    ) -> Route {
        if waypoints.is_empty() {
            return Route::new(route_id, Vec::new());
        }

        if waypoints.len() == 1 {
            return Route::new(route_id, vec![waypoints[0].clone()]);
        }

        let mut planned_waypoints = Vec::new();

        for pair in waypoints.windows(2) {
            let mut planned_segment = Self::plan_segment(&pair[0], &pair[1], hazards);

            // Every planned segment contains its start and end. The start of
            // each later segment duplicates the previous segment's end.
            if !planned_waypoints.is_empty() {
                planned_segment.remove(0);
            }

            planned_waypoints.extend(planned_segment);
        }

        Route::new(route_id, planned_waypoints)
    }

    pub fn plan_segment(start: &Waypoint, end: &Waypoint, hazards: &[Hazard]) -> Vec<Waypoint> {
        let direct_segment = LineSegment::new(start.position, end.position);

        let mut blocking_hazards: Vec<&Hazard> = hazards
            .iter()
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

    let planned = RoutePlanner::plan_segment(&start, &end, &[hazard.clone()]);

    assert!(planned.len() > 2);

    for pair in planned.windows(2) {
        let segment = LineSegment::new(pair[0].position, pair[1].position);

        assert!(
            !hazard.footprint.intersects_line_segment(&segment),
            "planned segment still intersects the hazard"
        );
    }
}
