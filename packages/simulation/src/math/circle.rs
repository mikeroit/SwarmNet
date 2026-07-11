use crate::math::Point2;

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

    pub fn contains_point(&self, point: &Point2) -> bool  {
        self.center.distance_to(point) <= self.radius
    }

    pub fn intersects_circle(&self, other: &Circle) -> bool {
        self.center.distance_to(&other.center) <= self.radius + other.radius
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
}
