use crate::prelude::{
  VertexArray,
  BufferObject,
  Texture,
  generate_textured_vertex_data
};

pub struct Quad {
  vertex_array: VertexArray
}

impl Quad {
  pub fn textured(width: u32, height: u32) -> Self {
    let (vertex_data, element_data) = generate_textured_vertex_data(width, height);
    let vertex_buffer = BufferObject::vertex_array(vertex_data);
    let element_buffer = BufferObject::element_array(element_data);
    let vertex_array = VertexArray::textured(vertex_buffer, element_buffer);

    Self {
      vertex_array
    }
  }
}

pub fn render_textured_quad(quad: &Quad, texture: &Texture) {
  texture.bind();
  quad.vertex_array.bind();
  
  unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null()); }
  
  quad.vertex_array.unbind();
  texture.unbind();
}