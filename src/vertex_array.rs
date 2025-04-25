use crate::prelude::BufferObject;

pub struct VertexArray {
  id: gl::types::GLuint
}

impl VertexArray {
  pub fn textured(vertex_buffer: BufferObject, element_buffer: BufferObject) -> Self {
    let mut id: gl::types::GLuint = 0;

    unsafe {
      gl::GenVertexArrays(1, &mut id);
      gl::BindVertexArray(id);
      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer.id());
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer.id());

      gl::EnableVertexAttribArray(0);
      gl::VertexAttribPointer(
        0,
        2,
        gl::FLOAT,
        gl::FALSE,
        (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
        std::ptr::null()
      );

      gl::EnableVertexAttribArray(1);
      gl::VertexAttribPointer(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
        (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
      );

      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      gl::BindVertexArray(0);
    }

    Self { id }
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindVertexArray(self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindVertexArray(0);
    }
  }
}

impl Drop for VertexArray {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteVertexArrays(1, &self.id);
    }
  }
}