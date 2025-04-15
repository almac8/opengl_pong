pub struct Vector2 {
  pub x: f32,
  pub y: f32
}

impl Vector2 {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
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
}