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

  pub fn normalized(&self) -> Self {
    Self {
      x: self.x / self.length(),
      y: self.y / self.length()
    }
  }

  fn length(&self) -> f32 {
    f32::sqrt((self.x * self.x) + (self.y * self.y))
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

  #[test]
  fn vector_length() {
    let input_x = 4.0;
    let input_y = 8.0;

    let square_x = input_x * input_x;
    let square_y = input_y * input_y;
    let square_sum = square_x + square_y;

    let expected_length = f32::sqrt(square_sum);
    
    let input_vector = Vector2::new(input_x, input_y);
    let vector_length = input_vector.length();

    assert_eq!(vector_length, expected_length);
  }

  #[test]
  fn normalization() {
    let input_x = 4.0;
    let input_y = 8.0;
    let expected_input_length = 8.944272;
    let expected_output_length = 0.99999994;

    let expected_output_x = input_x / expected_input_length;
    let expected_output_y = input_y / expected_input_length;

    let input_vector = Vector2::new(input_x, input_y);
    let input_length = input_vector.length();
    assert_eq!(input_length, expected_input_length);

    let output_vector = input_vector.normalized();
    assert_eq!(output_vector.x, expected_output_x);
    assert_eq!(output_vector.y, expected_output_y);

    assert_eq!(output_vector.length(), expected_output_length);
  }
}