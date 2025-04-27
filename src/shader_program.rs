use std::ffi::CString;

use crate::prelude::{Shader, Matrix4};

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
}

impl Drop for ShaderProgram {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.id);
    }
  }
}

pub fn set_model_matrix(shader_program: &ShaderProgram, matrix: &Matrix4) -> Result<(), String> {
  set_uniform_mat4(shader_program, "model", matrix)?;

  Ok(())
}

pub fn set_view_matrix(shader_program: &ShaderProgram, matrix: &Matrix4) -> Result<(), String> {
  set_uniform_mat4(shader_program, "view", matrix)?;

  Ok(())
}

pub fn set_projection_matrix(shader_program: &ShaderProgram, matrix: &Matrix4) -> Result<(), String> {
  set_uniform_mat4(shader_program, "projection", matrix)?;

  Ok(())
}

fn set_uniform_mat4(shader_program: &ShaderProgram, uniform_name: &str, matrix: &Matrix4) -> Result<(), String>{
  let uniform_name = CString::new(uniform_name).map_err(|error| error.to_string())?;
  let uniform_location = unsafe { gl::GetUniformLocation(shader_program.id(), uniform_name.as_ptr()) };
  unsafe { gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, matrix.flatten().as_ptr()); }

  Ok(())
}