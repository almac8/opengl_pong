use crate::{math::Vector2, prelude::Location};

pub struct Collider {
  location: Location,
  width: f32,
  height: f32
}

impl Collider {
  pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
    Self {
      location: Location::new(x, y),
      width,
      height
    }
  }

  pub fn location(&self) -> &Location {
    &self.location
  }

  pub fn width(&self) -> f32 {
    self.width
  }

  pub fn height(&self) -> f32 {
    self.height
  }

  pub fn set_location(&mut self, location: &Location) {
    self.location.set(Vector2::new(location.x(), location.y()));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn constructor() {
    let location_x = 32.0;
    let location_y = 32.0;
    let width = 64.0;
    let height = 32.0;

    let collider = Collider::new(location_x, location_y, width, height);

    assert_eq!(collider.location.x(), 32.0);
    assert_eq!(collider.location.y(), 32.0);
    assert_eq!(collider.width, 64.0);
    assert_eq!(collider.height, 32.0);
  }

  #[test]
  fn setting_location() {
    let location_x = 0.0;
    let location_y = 0.0;
    let width = 4.0;
    let height = 8.0;

    let new_location_x = 2.0;
    let new_location_y = 2.0;

    let new_location = Location::new(new_location_x, new_location_y);

    let mut collider = Collider::new(location_x, location_y, width, height);
    collider.set_location(&new_location);

    assert_eq!(collider.location.x(), new_location_x);
    assert_eq!(collider.location.y(), new_location_y);
  }
}