use std::{ffi::CString, path::Path};
use sdl2::event::Event;

fn main() -> Result<(), String> {
  let window_width = 800;
  let window_height = 600;

  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let mut event_pump = sdl_context.event_pump()?;

  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 3);

  let window = video_subsystem
    .window("Pong", window_width, window_height)
    .opengl()
    .build()
    .map_err(|error| error.to_string())?;

  let _gl_context = window.gl_create_context()?;
  let _gl = gl::load_with(|procname| video_subsystem.gl_get_proc_address(procname) as *const gl::types::GLvoid);

  let mut is_running = true;

  let vertices: Vec<f32> = vec![
    -8.0,  8.0,  0.0, 0.0,
     8.0,  8.0,  1.0, 0.0,
     8.0, -8.0,  1.0, 1.0,
    -8.0, -8.0,  0.0, 1.0
  ];

  let mut vertex_array_buffer: gl::types::GLuint = 0;
  unsafe {
    gl::GenBuffers(1, &mut vertex_array_buffer);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_array_buffer);

    gl::BufferData(
      gl::ARRAY_BUFFER,
      (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
      vertices.as_ptr() as *const gl::types::GLvoid,
      gl::STATIC_DRAW
    );

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  }

  let element_data: Vec<i32> = vec![
    0, 1, 2,
    0, 2, 3
  ];

  let mut element_buffer_object: gl::types::GLuint = 0;
  unsafe {
    gl::GenBuffers(1, &mut element_buffer_object);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_object);

    gl::BufferData(
      gl::ELEMENT_ARRAY_BUFFER,
      (element_data.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
      element_data.as_ptr() as *const gl::types::GLvoid,
      gl::STATIC_DRAW
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
  }

  let mut vertex_array_object: gl::types::GLuint = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut vertex_array_object);
    gl::BindVertexArray(vertex_array_object);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_array_buffer);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_object);

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

  let vertex_shader_source = CString::new(
    include_str!("vertex_shader.glsl")
  ).map_err(|error| error.to_string())?;

  let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
  unsafe {
    gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
    gl::CompileShader(vertex_shader);
  }

  let fragment_shader_source = CString::new(
    include_str!("fragment_shader.glsl")
  ).map_err(|error| error.to_string())?;

  let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
  unsafe {
    gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), std::ptr::null());
    gl::CompileShader(fragment_shader);
  }

  let shader_program = unsafe { gl::CreateProgram() };
  unsafe {
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);

    gl::LinkProgram(shader_program);

    gl::DetachShader(shader_program, vertex_shader);
    gl::DetachShader(shader_program, fragment_shader);

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);
  }

  let ball_image = image::open(Path::new("res/ball.png")).map_err(|error| error.to_string())?;

  let mut ball_texture: gl::types::GLuint = 0;
  unsafe {
    gl::GenTextures(1, &mut ball_texture);
    gl::BindTexture(gl::TEXTURE_2D, ball_texture);

    gl::TexImage2D(
      gl::TEXTURE_2D,
      0,
      gl::RGBA as gl::types::GLint,
      ball_image.width() as gl::types::GLint,
      ball_image.height() as gl::types::GLint,
      0,
      gl::RGBA,
      gl::UNSIGNED_BYTE,
      ball_image.as_bytes().as_ptr() as *const gl::types::GLvoid
    );

    gl::GenerateMipmap(gl::TEXTURE_2D);
  }

  let model_matrix = Matrix4::identity();
  let view_matrix = Matrix4::identity();
  let projection_matrix = Matrix4::orthographic(0.0, window_width as f32, window_height as f32, 0.0, -1.0, 1.0);

  unsafe {
    gl::Viewport(0, 0, window_width as i32, window_height as i32);
    gl::ClearColor(1.0, 0.5, 1.0, 1.0);
    gl::UseProgram(shader_program);

    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
  }

  let view_uniform_name = CString::new("view").map_err(|error| error.to_string())?;
  let view_uniform_location = unsafe { gl::GetUniformLocation(shader_program, view_uniform_name.as_ptr()) };
  unsafe { gl::UniformMatrix4fv(view_uniform_location, 1, gl::FALSE, view_matrix.flatten().as_ptr()); }

  let projection_uniform_name = CString::new("projection").map_err(|error| error.to_string())?;
  let projection_uniform_location = unsafe { gl::GetUniformLocation(shader_program, projection_uniform_name.as_ptr()) };
  unsafe { gl::UniformMatrix4fv(projection_uniform_location, 1, gl::FALSE, projection_matrix.flatten().as_ptr()); }

  while is_running {
    for event in event_pump.poll_iter() {
      if let Event::Quit { .. } = event { is_running = false }
    }
    
    let model_uniform_name = CString::new("model").map_err(|error| error.to_string())?;
    let model_uniform_location = unsafe { gl::GetUniformLocation(shader_program, model_uniform_name.as_ptr()) };
    
    unsafe { gl::UniformMatrix4fv(model_uniform_location, 1, gl::FALSE, model_matrix.flatten().as_ptr()); }

    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);

      gl::BindTexture(gl::TEXTURE_2D, ball_texture);
      gl::BindVertexArray(vertex_array_object);

      gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

      gl::BindVertexArray(0);
    }

    window.gl_swap_window();
  }

  Ok(())
}

struct Matrix4 {
  x: Vector4,
  y: Vector4,
  z: Vector4,
  w: Vector4,
}

impl Matrix4 {
  fn identity() -> Self {
    Self {
      x: Vector4::new(1.0, 0.0, 0.0, 0.0),
      y: Vector4::new(0.0, 1.0, 0.0, 0.0),
      z: Vector4::new(0.0, 0.0, 1.0, 0.0),
      w: Vector4::new(0.0, 0.0, 0.0, 1.0)
    }
  }

  fn orthographic(left: f32, right: f32, bottom:f32, top: f32, near: f32, far: f32) -> Self {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let fmn = far - near;
    let fpn = far + near;

    Self {
      x: Vector4::new(2.0 / rml, 0.0, 0.0, -(rpl / rml)),
      y: Vector4::new(0.0, 2.0 / tmb, 0.0, -(tpb / tmb)),
      z: Vector4::new(0.0, 0.0, -2.0 / fmn, -(fpn / fmn)),
      w: Vector4::new(0.0, 0.0, 0.0, 1.0)
    }
  }

  fn flatten(&self) -> Vec<f32> {
    vec![
      self.x.x, self.y.x, self.z.x, self.w.x,
      self.x.y, self.y.y, self.z.y, self.w.y,
      self.x.z, self.y.z, self.z.z, self.w.z,
      self.x.w, self.y.w, self.z.w, self.w.w
    ]
  }
}

struct Vector4 {
  x: f32,
  y: f32,
  z: f32,
  w: f32,
}

impl Vector4 {
  fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self { x, y, z, w }
  }
}