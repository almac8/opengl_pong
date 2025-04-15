pub fn generate_textured_vertex_data(width: u32, height: u32) -> (Vec<f32>, Vec<i32>) {
  let hw = width as f32 / 2.0;
  let hh = height as f32 / 2.0;

  (
    vec![
      -hw,  hh,  0.0, 0.0,
       hw,  hh,  1.0, 0.0,
       hw, -hh,  1.0, 1.0,
      -hw, -hh,  0.0, 1.0
    ],

    vec![
      0, 1, 2,
      0, 2, 3
    ]
  )
}

#[cfg(test)]
mod tests {
  use super::generate_textured_vertex_data;

  #[test]
  fn generated_data() {
    let width = 64;
    let height = 64;

    let (
      vertex_data,
      element_data
    ) = generate_textured_vertex_data(width, height);

    assert_eq!(vertex_data, vec![
      -32.0,  32.0, 0.0, 0.0,
       32.0,  32.0, 1.0, 0.0,
       32.0, -32.0, 1.0, 1.0,
      -32.0, -32.0, 0.0, 1.0
    ]);

    assert_eq!(element_data, vec![
      0, 1, 2,
      0, 2, 3
    ]);
  }
}