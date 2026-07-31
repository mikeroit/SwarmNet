use crate::math::Point2;
use derive_new::new;

#[derive(new)]
pub struct LineSegment {
    pub start: Point2,
    pub end: Point2,
}
