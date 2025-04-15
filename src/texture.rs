use std::path::Path;

pub struct Texture {
  id: gl::types::GLuint
}

impl Texture {
  pub fn load(file_path: &Path) -> Result<Self, String> {
    let raw_image = image::open(file_path).map_err(|error| error.to_string())?;
    
    let mut id: gl::types::GLuint = 0;
    unsafe {
      gl::GenTextures(1, &mut id);
      gl::BindTexture(gl::TEXTURE_2D, id);

      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as gl::types::GLint,
        raw_image.width() as gl::types::GLint,
        raw_image.height() as gl::types::GLint,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        raw_image.as_bytes().as_ptr() as *const gl::types::GLvoid
      );

      gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    Ok(
      Self { id }
    )
  }

  pub fn id(&self) -> gl::types::GLuint {
    self.id
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteTextures(1, &mut self.id);
    }
  }
}