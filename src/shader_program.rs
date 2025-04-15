use crate::prelude::Shader;

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