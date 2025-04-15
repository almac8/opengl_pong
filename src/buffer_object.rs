pub struct BufferObject {
  id: gl::types::GLuint
}

impl BufferObject {
  pub fn vertex_array(vertex_data: Vec<f32>) -> Self {
    let mut id: gl::types::GLuint = 0;

    unsafe {
      gl::GenBuffers(1, &mut id);
      gl::BindBuffer(gl::ARRAY_BUFFER, id);

      gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertex_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
        vertex_data.as_ptr() as *const gl::types::GLvoid,
        gl::STATIC_DRAW
      );

      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    Self { id }
  }

  pub fn element_array(element_data: Vec<i32>) -> Self {
    let mut id: gl::types::GLuint = 0;

    unsafe {
      gl::GenBuffers(1, &mut id);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);

      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (element_data.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
        element_data.as_ptr() as *const gl::types::GLvoid,
        gl::STATIC_DRAW
      );

      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }
    Self { id }
  }

  pub fn id(&self) -> gl::types::GLuint {
    self.id
  }
}

impl Drop for BufferObject {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteBuffers(1, &mut self.id);
    }
  }
}