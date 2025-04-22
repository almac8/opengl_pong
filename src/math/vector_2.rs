use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32
}

impl Vector2 {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }
}

impl Mul<f32> for Vector2 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Self {
      x: self.x * rhs,
      y: self.y * rhs
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn constructor() {
    let location_x = 32.0;
    let location_y = 32.0;

    let vector = Vector2::new(location_x, location_y);

    assert_eq!(vector.x, location_x);
    assert_eq!(vector.y, location_y);
  }

  #[test]
  fn multiplication() {
    let x_value = 2.0;
    let y_value = 4.0;
    let multiplication_factor = 8.0;

    let vector = Vector2::new(x_value, y_value);
    let transformed_vector = vector * multiplication_factor;

    assert_eq!(transformed_vector.x, x_value * multiplication_factor);
    assert_eq!(transformed_vector.y, y_value * multiplication_factor);
  }
}