pub struct Vector3 {
  x: f32,
  y: f32,
  z: f32
}

impl Vector3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }
}

#[cfg(test)]
mod tests {
  use super::Vector3;

  #[test]
  fn constructor() {
    let location_x = 32.0;
    let location_y = 32.0;
    let location_z = 0.0;

    let vector = Vector3::new(location_x, location_y, location_z);

    assert_eq!(vector.x, location_x);
    assert_eq!(vector.y, location_y);
    assert_eq!(vector.z, location_z);
  }
}