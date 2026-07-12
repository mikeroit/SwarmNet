use std::ops::{Add, Div, Mul, Sub, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let length = self.length();

        if length == 0.0 {
            return Self::new(0.0, 0.0);
        }

        Self::new(self.x / length, self.y / length)
    }

    pub fn dot(&self, other: &Vector2) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Vector2) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Vector2) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, multiplicand: f64) -> Self::Output {
        Self::new(self.x * multiplicand, self.y * multiplicand)
    }
}

impl Div<f64> for Vector2 {
    type Output = Self;

    fn div(self, multiplicand: f64) -> Self::Output {
        Self::new(self.x / multiplicand, self.y / multiplicand)
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_length_uses_pythagorean_distance() {
        let vector = Vector2::new(3.0, 4.0);

        assert_eq!(vector.length(), 5.0);
    }
}
