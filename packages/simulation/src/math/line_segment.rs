use crate::math::Point2;

pub struct LineSegment {
    pub start: Point2,
    pub end: Point2,
}

impl LineSegment {
    pub fn new(start: Point2, end: Point2) -> Self {
        LineSegment {
            start,
            end,
        }
    }
}
