use std::ffi::CString;

use crate::prelude::{ Shader, Matrix4 };

pub struct ShaderProgram {
  id: gl::types::GLuint
}

impl ShaderProgram {
  pub fn link(shaders: Vec<Shader>) -> Self {
    let id = unsafe { gl::CreateProgram() };

    for shader in &shaders {
      unsafe {
        gl::AttachShader(id, shader.id());
      }
    }

    unsafe {
      gl::LinkProgram(id);
    }

    for shader in &shaders {
      unsafe {
        gl::DetachShader(id, shader.id());
      }
    }

    Self { id }
  }

  pub fn id(&self) -> gl::types::GLuint {
    self.id
  }

  pub fn set_view_matrix(&self, matrix: Matrix4) -> Result<(), String> {
    self.set_uniform_mat4("view", matrix)
  }

  pub fn set_projection_matrix(&self, matrix: Matrix4) -> Result<(), String> {
    self.set_uniform_mat4("projection", matrix)
  }

  fn set_uniform_mat4(&self, uniform_name: &str, matrix: Matrix4) -> Result<(), String>{
    let projection_uniform_name = CString::new(uniform_name).map_err(|error| error.to_string())?;
    let projection_uniform_location = unsafe { gl::GetUniformLocation(self.id(), projection_uniform_name.as_ptr()) };
    unsafe { gl::UniformMatrix4fv(projection_uniform_location, 1, gl::FALSE, matrix.flatten().as_ptr()); }

    Ok(())
  }
}

impl Drop for ShaderProgram {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.id);
    }
  }
}