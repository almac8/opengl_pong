#[derive(Clone, Copy)]
pub struct Vector4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32
}

impl Vector4 {
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self { x, y, z, w }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn constructor() {
    let x = 0.0;
    let y = 0.0;
    let z = 0.0;
    let w = 0.0;

    let vec4 = Vector4::new(x, y, z, w);

    assert_eq!(vec4.x, x);
    assert_eq!(vec4.y, y);
    assert_eq!(vec4.z, z);
    assert_eq!(vec4.w, w);
  }
}