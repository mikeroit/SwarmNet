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

    pub fn scaled(&self, scale: f64) -> Self {
        Self::new(self.x * scale, self.y * scale)
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
