use crate::math::{LineSegment, Point2};

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub center: Point2,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point2, radius: f64) -> Self {
        assert!(radius >= 0.0, "circle radius must be non-negative");
        Self { center, radius }
    }

    pub fn contains_point(&self, point: &Point2) -> bool {
        self.center.distance_to(point) <= self.radius
    }

    pub fn intersects_circle(&self, other: &Circle) -> bool {
        self.center.distance_to(&other.center) <= self.radius + other.radius
    }

    pub fn intersects_line_segment(&self, line_segment: &LineSegment) -> bool {
        let ab = line_segment.end - line_segment.start;
        let ac = self.center - line_segment.start;

        let length_squared = ab.length_squared();

        if length_squared == 0.0 {
            return self.contains_point(&line_segment.start);
        }

        let t = (ac.dot(&ab) / length_squared).clamp(0.0, 1.0);
        let closest = line_segment.start + (ab * t);

        self.contains_point(&closest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_point_inside_circle() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);

        assert!(circle.contains_point(&Point2::new(3.0, 4.0)));
    }

    #[test]
    fn contains_point_on_boundary() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);

        assert!(circle.contains_point(&Point2::new(5.0, 0.0)));
    }

    #[test]
    fn does_not_contain_point_outside_circle() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);

        assert!(!circle.contains_point(&Point2::new(5.01, 0.0)));
    }

    #[test]
    fn tangent_circles_intersect() {
        let first = Circle::new(Point2::new(0.0, 0.0), 5.0);
        let second = Circle::new(Point2::new(10.0, 0.0), 5.0);

        assert!(first.intersects_circle(&second));
    }

    #[test]
    fn separated_circles_do_not_intersect() {
        let first = Circle::new(Point2::new(0.0, 0.0), 5.0);
        let second = Circle::new(Point2::new(10.01, 0.0), 5.0);

        assert!(!first.intersects_circle(&second));
    }

    #[test]
    fn zero_length_segment_inside_circle_intersects() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);
        let segment = LineSegment {
            start: Point2::new(3.0, 4.0),
            end: Point2::new(3.0, 4.0),
        };

        assert!(circle.intersects_line_segment(&segment));
    }

    #[test]
    fn zero_length_segment_outside_circle_does_not_intersect() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);
        let segment = LineSegment {
            start: Point2::new(6.0, 0.0),
            end: Point2::new(6.0, 0.0),
        };

        assert!(!circle.intersects_line_segment(&segment));
    }

    #[test]
    fn segment_through_circle_center_intersects() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);
        let segment = LineSegment {
            start: Point2::new(-10.0, 0.0),
            end: Point2::new(10.0, 0.0),
        };

        assert!(circle.intersects_line_segment(&segment));
    }

    #[test]
    fn segment_missing_circle_does_not_intersect() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);
        let segment = LineSegment {
            start: Point2::new(-10.0, 6.0),
            end: Point2::new(10.0, 6.0),
        };

        assert!(!circle.intersects_line_segment(&segment));
    }

    #[test]
    fn tangent_segment_intersects_circle() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);
        let segment = LineSegment {
            start: Point2::new(-10.0, 5.0),
            end: Point2::new(10.0, 5.0),
        };

        assert!(circle.intersects_line_segment(&segment));
    }

    #[test]
    fn segment_ending_before_circle_does_not_intersect() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);
        let segment = LineSegment {
            start: Point2::new(-10.0, 0.0),
            end: Point2::new(-6.0, 0.0),
        };

        assert!(!circle.intersects_line_segment(&segment));
    }

    #[test]
    fn segment_endpoint_on_circle_intersects() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);
        let segment = LineSegment {
            start: Point2::new(-10.0, 0.0),
            end: Point2::new(-5.0, 0.0),
        };

        assert!(circle.intersects_line_segment(&segment));
    }

    #[test]
    fn segment_fully_inside_circle_intersects() {
        let circle = Circle::new(Point2::new(0.0, 0.0), 5.0);
        let segment = LineSegment {
            start: Point2::new(-1.0, 0.0),
            end: Point2::new(1.0, 0.0),
        };

        assert!(circle.intersects_line_segment(&segment));
    }
}
