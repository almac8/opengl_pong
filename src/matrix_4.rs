use crate::vector_4::Vector4;

pub struct Matrix4 {
  x: Vector4,
  y: Vector4,
  z: Vector4,
  w: Vector4,
}

impl Matrix4 {
  pub fn identity() -> Self {
    Self {
      x: Vector4::new(1.0, 0.0, 0.0, 0.0),
      y: Vector4::new(0.0, 1.0, 0.0, 0.0),
      z: Vector4::new(0.0, 0.0, 1.0, 0.0),
      w: Vector4::new(0.0, 0.0, 0.0, 1.0)
    }
  }

  pub fn orthographic(left: f32, right: f32, bottom:f32, top: f32, near: f32, far: f32) -> Self {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let fmn = far - near;
    let fpn = far + near;

    Self {
      x: Vector4::new(2.0 / rml, 0.0, 0.0, -(rpl / rml)),
      y: Vector4::new(0.0, 2.0 / tmb, 0.0, -(tpb / tmb)),
      z: Vector4::new(0.0, 0.0, -2.0 / fmn, -(fpn / fmn)),
      w: Vector4::new(0.0, 0.0, 0.0, 1.0)
    }
  }

  pub fn flatten(&self) -> Vec<f32> {
    vec![
      self.x.x, self.y.x, self.z.x, self.w.x,
      self.x.y, self.y.y, self.z.y, self.w.y,
      self.x.z, self.y.z, self.z.z, self.w.z,
      self.x.w, self.y.w, self.z.w, self.w.w
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::Matrix4;

  #[test]
  fn identity_constructor() {
    let mat4 = Matrix4::identity();

    assert_eq!(mat4.x.x, 1.0);
    assert_eq!(mat4.x.y, 0.0);
    assert_eq!(mat4.x.z, 0.0);
    assert_eq!(mat4.x.w, 0.0);

    assert_eq!(mat4.y.x, 0.0);
    assert_eq!(mat4.y.y, 1.0);
    assert_eq!(mat4.y.z, 0.0);
    assert_eq!(mat4.y.w, 0.0);

    assert_eq!(mat4.z.x, 0.0);
    assert_eq!(mat4.z.y, 0.0);
    assert_eq!(mat4.z.z, 1.0);
    assert_eq!(mat4.z.w, 0.0);

    assert_eq!(mat4.w.x, 0.0);
    assert_eq!(mat4.w.y, 0.0);
    assert_eq!(mat4.w.z, 0.0);
    assert_eq!(mat4.w.w, 1.0);
  }

  #[test]
  fn orthographic_constructor() {
    let screen_width = 800.0;
    let screen_height = 600.0;

    let left = 0.0;
    let right = screen_width;
    let bottom = screen_height;
    let top = 0.0;
    let near = -1.0;
    let far = 1.0;

    let mat4 = Matrix4::orthographic(left, right, bottom, top, near, far);
    
    assert_eq!(mat4.x.x, 0.0025);
    assert_eq!(mat4.x.y, 0.0);
    assert_eq!(mat4.x.z, 0.0);
    assert_eq!(mat4.x.w, -1.0);
    
    assert_eq!(mat4.y.x, 0.0);
    assert_eq!(mat4.y.y, -0.0033333334);
    assert_eq!(mat4.y.z, 0.0);
    assert_eq!(mat4.y.w, 1.0);

    assert_eq!(mat4.z.x, 0.0);
    assert_eq!(mat4.z.y, 0.0);
    assert_eq!(mat4.z.z, -1.0);
    assert_eq!(mat4.z.w, 0.0);

    assert_eq!(mat4.w.x, 0.0);
    assert_eq!(mat4.w.y, 0.0);
    assert_eq!(mat4.w.z, 0.0);
    assert_eq!(mat4.w.w, 1.0);
  }
  
  #[test]
  fn flatten() {
    let matrix = Matrix4::identity();
    let flattened_matrix = matrix.flatten();

    assert_eq!(flattened_matrix.len(), 16);

    assert_eq!(flattened_matrix[0], 1.0);
    assert_eq!(flattened_matrix[1], 0.0);
    assert_eq!(flattened_matrix[2], 0.0);
    assert_eq!(flattened_matrix[3], 0.0);

    assert_eq!(flattened_matrix[4], 0.0);
    assert_eq!(flattened_matrix[5], 1.0);
    assert_eq!(flattened_matrix[6], 0.0);
    assert_eq!(flattened_matrix[7], 0.0);

    assert_eq!(flattened_matrix[8], 0.0);
    assert_eq!(flattened_matrix[9], 0.0);
    assert_eq!(flattened_matrix[10], 1.0);
    assert_eq!(flattened_matrix[11], 0.0);

    assert_eq!(flattened_matrix[12], 0.0);
    assert_eq!(flattened_matrix[13], 0.0);
    assert_eq!(flattened_matrix[14], 0.0);
    assert_eq!(flattened_matrix[15], 1.0);
  }
}
