use std::{ffi::CString, fs, path::Path};

pub struct Shader {
  id: gl::types::GLuint
}

impl Shader {
  pub fn vertex(file_path: &Path) -> Result<Self, String> {
    let shader_source = CString::new(
      fs::read_to_string(file_path).map_err(|error| error.to_string())?
    ).map_err(|error| error.to_string())?;
  
    let id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
      gl::ShaderSource(id, 1, &shader_source.as_ptr(), std::ptr::null());
      gl::CompileShader(id);
    }

    Ok(
      Self { id }
    )
  }

  pub fn fragment(file_path: &Path) -> Result<Self, String> {
    let shader_source = CString::new(
      fs::read_to_string(file_path).map_err(|error| error.to_string())?
    ).map_err(|error| error.to_string())?;
  
    let id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
      gl::ShaderSource(id, 1, &shader_source.as_ptr(), std::ptr::null());
      gl::CompileShader(id);
    }

    Ok(
      Self { id }
    )
  }

  pub fn id(&self) -> gl::types::GLuint {
    self.id
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteShader(self.id);
    }
  }
}