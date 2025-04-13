use std::ffi::CString;
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
    -0.5, -0.5,
     0.5, -0.5,
     0.0,  0.5
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

  let mut vertex_array_object: gl::types::GLuint = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut vertex_array_object);
    gl::BindVertexArray(vertex_array_object);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_array_buffer);

    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
      0,
      2,
      gl::FLOAT,
      gl::FALSE,
      (2 * std::mem::size_of::<f32>()) as gl::types::GLint,
      std::ptr::null()
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

  unsafe {
    gl::Viewport(0, 0, window_width as i32, window_height as i32);
    gl::ClearColor(1.0, 0.5, 1.0, 1.0);
    gl::UseProgram(shader_program);
  }

  while is_running {
    for event in event_pump.poll_iter() {
      if let Event::Quit { .. } = event { is_running = false }
    }

    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);

      gl::BindVertexArray(vertex_array_object);
      gl::DrawArrays(gl::TRIANGLES, 0, 3);
      gl::BindVertexArray(0);
    }

    window.gl_swap_window();
  }

  Ok(())
}