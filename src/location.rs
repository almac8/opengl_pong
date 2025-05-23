use crate::prelude::{Matrix4, Vector2};

pub struct Location {
  x: f32,
  y: f32,
  matrix: Matrix4
}

impl Location {
  pub fn new(x: f32, y: f32) -> Self {
    let mut matrix = Matrix4::identity();

    matrix.x.w = x;
    matrix.y.w = y;

    Self {
      x,
      y,
      matrix
    }
  }

  pub fn x(&self) -> f32 {
    self.x
  }

  pub fn y(&self) -> f32 {
    self.y
  }

  pub fn matrix(&self) -> &Matrix4 {
    &self.matrix
  }

  pub fn translate(&mut self, translation_vector: Vector2) {
    self.x += translation_vector.x;
    self.y += translation_vector.y;
    
    self.matrix.x.w = self.x;
    self.matrix.y.w = self.y;
  }

  pub fn set(&mut self, new_location: Vector2) {
    self.x = new_location.x;
    self.y = new_location.y;
    
    self.matrix.x.w = self.x;
    self.matrix.y.w = self.y;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn constructed_at() {
    let location_x = 32.0;
    let location_y = 64.0;

    let location = Location::new(location_x, location_y);

    assert_eq!(location.x, location_x);
    assert_eq!(location.y, location_y);
    
    assert_eq!(location.matrix.x.x, 1.0);
    assert_eq!(location.matrix.x.y, 0.0);
    assert_eq!(location.matrix.x.z, 0.0);
    assert_eq!(location.matrix.x.w, location_x);

    assert_eq!(location.matrix.y.x, 0.0);
    assert_eq!(location.matrix.y.y, 1.0);
    assert_eq!(location.matrix.y.z, 0.0);
    assert_eq!(location.matrix.y.w, location_y);

    assert_eq!(location.matrix.z.x, 0.0);
    assert_eq!(location.matrix.z.y, 0.0);
    assert_eq!(location.matrix.z.z, 1.0);
    assert_eq!(location.matrix.z.w, 0.0);

    assert_eq!(location.matrix.w.x, 0.0);
    assert_eq!(location.matrix.w.y, 0.0);
    assert_eq!(location.matrix.w.z, 0.0);
    assert_eq!(location.matrix.w.w, 1.0);
  }

  #[test]
  fn translation() {
    let location_x = 32.0;
    let location_y = 64.0;
    let translation_x = 32.0;
    let translation_y = 64.0;
    let translation_vector = Vector2::new(translation_x, translation_y);

    let mut location = Location::new(location_x, location_y);

    assert_eq!(location.x, location_x);
    assert_eq!(location.y, location_y);
    
    assert_eq!(location.matrix.x.x, 1.0);
    assert_eq!(location.matrix.x.y, 0.0);
    assert_eq!(location.matrix.x.z, 0.0);
    assert_eq!(location.matrix.x.w, location_x);

    assert_eq!(location.matrix.y.x, 0.0);
    assert_eq!(location.matrix.y.y, 1.0);
    assert_eq!(location.matrix.y.z, 0.0);
    assert_eq!(location.matrix.y.w, location_y);

    assert_eq!(location.matrix.z.x, 0.0);
    assert_eq!(location.matrix.z.y, 0.0);
    assert_eq!(location.matrix.z.z, 1.0);
    assert_eq!(location.matrix.z.w, 0.0);

    assert_eq!(location.matrix.w.x, 0.0);
    assert_eq!(location.matrix.w.y, 0.0);
    assert_eq!(location.matrix.w.z, 0.0);
    assert_eq!(location.matrix.w.w, 1.0);

    location.translate(translation_vector);

    assert_eq!(location.x, location_x + translation_x);
    assert_eq!(location.y, location_y + translation_y);
    
    assert_eq!(location.matrix.x.x, 1.0);
    assert_eq!(location.matrix.x.y, 0.0);
    assert_eq!(location.matrix.x.z, 0.0);
    assert_eq!(location.matrix.x.w, location_x + translation_x);

    assert_eq!(location.matrix.y.x, 0.0);
    assert_eq!(location.matrix.y.y, 1.0);
    assert_eq!(location.matrix.y.z, 0.0);
    assert_eq!(location.matrix.y.w, location_y + translation_y);

    assert_eq!(location.matrix.z.x, 0.0);
    assert_eq!(location.matrix.z.y, 0.0);
    assert_eq!(location.matrix.z.z, 1.0);
    assert_eq!(location.matrix.z.w, 0.0);

    assert_eq!(location.matrix.w.x, 0.0);
    assert_eq!(location.matrix.w.y, 0.0);
    assert_eq!(location.matrix.w.z, 0.0);
    assert_eq!(location.matrix.w.w, 1.0);
  }

  #[test]
  fn setting() {
    let location_x = 2.0;
    let location_y = 4.0;
    let new_location_x = 4.0;
    let new_location_y = 2.0;

    let mut location = Location::new(location_x, location_y);

    assert_eq!(location.x, location_x);
    assert_eq!(location.y, location_y);
    
    assert_eq!(location.matrix.x.x, 1.0);
    assert_eq!(location.matrix.x.y, 0.0);
    assert_eq!(location.matrix.x.z, 0.0);
    assert_eq!(location.matrix.x.w, location_x);

    assert_eq!(location.matrix.y.x, 0.0);
    assert_eq!(location.matrix.y.y, 1.0);
    assert_eq!(location.matrix.y.z, 0.0);
    assert_eq!(location.matrix.y.w, location_y);

    assert_eq!(location.matrix.z.x, 0.0);
    assert_eq!(location.matrix.z.y, 0.0);
    assert_eq!(location.matrix.z.z, 1.0);
    assert_eq!(location.matrix.z.w, 0.0);

    assert_eq!(location.matrix.w.x, 0.0);
    assert_eq!(location.matrix.w.y, 0.0);
    assert_eq!(location.matrix.w.z, 0.0);
    assert_eq!(location.matrix.w.w, 1.0);

    location.set(Vector2::new(new_location_x, new_location_y));

    assert_eq!(location.x, new_location_x);
    assert_eq!(location.y, new_location_y);
    
    assert_eq!(location.matrix.x.x, 1.0);
    assert_eq!(location.matrix.x.y, 0.0);
    assert_eq!(location.matrix.x.z, 0.0);
    assert_eq!(location.matrix.x.w, new_location_x);

    assert_eq!(location.matrix.y.x, 0.0);
    assert_eq!(location.matrix.y.y, 1.0);
    assert_eq!(location.matrix.y.z, 0.0);
    assert_eq!(location.matrix.y.w, new_location_y);

    assert_eq!(location.matrix.z.x, 0.0);
    assert_eq!(location.matrix.z.y, 0.0);
    assert_eq!(location.matrix.z.z, 1.0);
    assert_eq!(location.matrix.z.w, 0.0);

    assert_eq!(location.matrix.w.x, 0.0);
    assert_eq!(location.matrix.w.y, 0.0);
    assert_eq!(location.matrix.w.z, 0.0);
    assert_eq!(location.matrix.w.w, 1.0);
  }
}