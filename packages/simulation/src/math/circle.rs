use crate::math::Point2;

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    center: Point2,
    radius: f32,
}

impl Circle {
    pub fn new(center: Point2, radius: f32) -> Self {
        Self {
            center,
            radius,
        }
    }
}
