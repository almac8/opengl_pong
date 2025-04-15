use std::{ffi::CString, fs, path::Path, time::{Duration, Instant}};
use sdl2::event::Event;

mod math;
mod vertex_data;
mod buffer_object;

mod prelude {
  pub use crate::math::Vector3;
  pub use crate::math::Vector4;
  pub use crate::math::Matrix4;

  pub use crate::vertex_data::generate_textured_vertex_data;

  pub use crate::buffer_object::BufferObject;
}

use prelude::BufferObject;

pub fn launch() -> Result<(), String> {
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

  let (ball_vertex_data, ball_element_data) = crate::prelude::generate_textured_vertex_data(16, 16);
  let (paddle_vertex_data, paddle_element_data) = crate::prelude::generate_textured_vertex_data(16, 128);

  let ball_vertex_array_buffer = BufferObject::vertex_array(ball_vertex_data);
  let paddle_vertex_array_buffer = BufferObject::vertex_array(paddle_vertex_data);

  let ball_element_buffer_object = BufferObject::element_array(ball_element_data);
  let paddle_element_buffer_object = BufferObject::element_array(paddle_element_data);
  
  let mut ball_vertex_array_object: gl::types::GLuint = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut ball_vertex_array_object);
    gl::BindVertexArray(ball_vertex_array_object);
    gl::BindBuffer(gl::ARRAY_BUFFER, ball_vertex_array_buffer.id());
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ball_element_buffer_object.id());

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

  let mut paddle_vertex_array_object: gl::types::GLuint = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut paddle_vertex_array_object);
    gl::BindVertexArray(paddle_vertex_array_object);
    gl::BindBuffer(gl::ARRAY_BUFFER, paddle_vertex_array_buffer.id());
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, paddle_element_buffer_object.id());

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
    fs::read_to_string(Path::new("res/shaders/vertex_shader.glsl")).map_err(|error| error.to_string())?
  ).map_err(|error| error.to_string())?;

  let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
  unsafe {
    gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
    gl::CompileShader(vertex_shader);
  }

  let fragment_shader_source = CString::new(
    fs::read_to_string(Path::new("res/shaders/fragment_shader.glsl")).map_err(|error| error.to_string())?
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
  
  let ball_image = image::open(Path::new("res/textures/ball.png")).map_err(|error| error.to_string())?;
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

  let paddle_image = image::open(Path::new("res/textures/paddle.png")).map_err(|error| error.to_string())?;
  let mut paddle_texture: gl::types::GLuint = 0;
  unsafe {
    gl::GenTextures(1, &mut paddle_texture);
    gl::BindTexture(gl::TEXTURE_2D, paddle_texture);

    gl::TexImage2D(
      gl::TEXTURE_2D,
      0,
      gl::RGBA as gl::types::GLint,
      paddle_image.width() as gl::types::GLint,
      paddle_image.height() as gl::types::GLint,
      0,
      gl::RGBA,
      gl::UNSIGNED_BYTE,
      paddle_image.as_bytes().as_ptr() as *const gl::types::GLvoid
    );

    gl::GenerateMipmap(gl::TEXTURE_2D);
  }

  let mut ball_model_matrix = prelude::Matrix4::identity();
  let mut paddle_model_matrix = prelude::Matrix4::identity();

  let view_matrix = prelude::Matrix4::identity();
  let projection_matrix = prelude::Matrix4::orthographic(0.0, window_width as f32, window_height as f32, 0.0, -1.0, 1.0);

  let ball_start_location = prelude::Vector3::new(window_width as f32 / 2.0, window_height as f32 / 2.0, 0.0);
  ball_model_matrix.translate(ball_start_location);
  
  let paddle_start_location = prelude::Vector3::new(32.0, window_height as f32 / 2.0, 0.0);
  paddle_model_matrix.translate(paddle_start_location);

  unsafe {
    gl::Viewport(0, 0, window_width as i32, window_height as i32);
    gl::ClearColor(0.2, 0.2, 0.4, 1.0);
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

  let mut ball_velocity_x = 0.5;
  let mut ball_velocity_y = 0.5;

  let mut current_time = Instant::now();
  let mut previous_time = current_time;
  while is_running {
    current_time = Instant::now();
    let deltatime = current_time - previous_time;
    previous_time = current_time;

    for event in event_pump.poll_iter() {
      if let Event::Quit { .. } = event { is_running = false }
    }

    let deltamillis = deltatime.as_millis() as f32;
    let translation_vector = prelude::Vector3::new(ball_velocity_x * deltamillis, ball_velocity_y * deltamillis, 0.0);
    ball_model_matrix.translate(translation_vector);

    if ball_model_matrix.x.w >= window_width as f32 || ball_model_matrix.x.w <= 0.0 {
      ball_velocity_x *= -1.0;
    }

    if ball_model_matrix.y.w >= window_height as f32 || ball_model_matrix.y.w <= 0.0 {
      ball_velocity_y *= -1.0;
    }
    
    let model_uniform_name = CString::new("model").map_err(|error| error.to_string())?;
    let model_uniform_location = unsafe { gl::GetUniformLocation(shader_program, model_uniform_name.as_ptr()) };
    
    unsafe { gl::UniformMatrix4fv(model_uniform_location, 1, gl::FALSE, ball_model_matrix.flatten().as_ptr()); }

    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);

      gl::BindTexture(gl::TEXTURE_2D, ball_texture);
      gl::BindVertexArray(ball_vertex_array_object);
      gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
      gl::BindVertexArray(0);
    }

    unsafe { gl::UniformMatrix4fv(model_uniform_location, 1, gl::FALSE, paddle_model_matrix.flatten().as_ptr()); }

    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, paddle_texture);
      gl::BindVertexArray(paddle_vertex_array_object);
      gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
      gl::BindVertexArray(0);
    }

    window.gl_swap_window();

    let target_fps = 60;

    let frame_max_duration = Duration::from_secs(1) / target_fps;

    let end_time = Instant::now();
    let frame_duration = end_time - previous_time;
    let time_left = frame_max_duration - frame_duration;

    std::thread::sleep(time_left);
  }

  Ok(())
}