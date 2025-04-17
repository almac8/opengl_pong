use crate::prelude::Location;

pub struct Collider {
  location: Location,
  width: f32,
  height: f32
}

impl Collider {
  pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
    Self {
      location: Location::at(x, y),
      width,
      height
    }
  }

  pub fn location(&self) -> Location {
    self.location
  }

  pub fn width(&self) -> f32 {
    self.width
  }

  pub fn height(&self) -> f32 {
    self.height
  }

  pub fn set_location(&mut self, location: &Location) {
    self.location = *location;
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
}