use std::path::Path;

use crate::prelude::{
  Texture,
  VertexArray,
  generate_textured_vertex_data,
  BufferObject
};

pub struct Sprite {
  texture: Texture,
  vertex_array: VertexArray
}

impl Sprite {
  pub fn new(file_path: &Path, width: u32, height: u32) -> Result<Self, String> {
    let texture = Texture::load(file_path)?;
    let (vertex_data, element_data) = generate_textured_vertex_data(width, height);
    let vertex_buffer = BufferObject::vertex_array(vertex_data);
    let element_buffer = BufferObject::element_array(element_data);
    let vertex_array = VertexArray::textured(vertex_buffer, element_buffer);

    Ok(
      Self {
        texture,
        vertex_array
      }
    )
  }

  pub fn render(&self) {
    self.texture.bind();
    self.vertex_array.bind();
    
    unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null()); }

    self.vertex_array.unbind();
    self.texture.unbind();
  }
}