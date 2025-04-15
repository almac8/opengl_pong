use std::{path::Path, time::{Duration, Instant}};
use sdl2::event::Event;

mod math;
mod vertex_data;
mod buffer_object;
mod vertex_array;
mod shader;
mod shader_program;
mod texture;
mod sprite;

mod prelude {
  pub use crate::math::Vector2;
  pub use crate::math::Vector4;
  pub use crate::math::Matrix4;
  pub use crate::vertex_data::generate_textured_vertex_data;
  pub use crate::buffer_object::BufferObject;
  pub use crate::vertex_array::VertexArray;
  pub use crate::shader::Shader;
  pub use crate::shader_program::ShaderProgram;
  pub use crate::texture::Texture;
  pub use crate::sprite::Sprite;
}

use prelude::{
  Vector2,
  Matrix4,
  Shader,
  ShaderProgram,
  Sprite
};

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

  let ball_sprite = Sprite::new(Path::new("res/textures/ball.png"), 16, 16)?;
  let paddle_sprite = Sprite::new(Path::new("res/textures/paddle.png"), 16, 128)?;

  let shader_program = ShaderProgram::link(
    vec![
      Shader::vertex(Path::new("res/shaders/vertex_shader.glsl"))?,
      Shader::fragment(Path::new("res/shaders/fragment_shader.glsl"))?
    ]
  );
  
  let mut ball_model_matrix = Matrix4::identity();
  let mut left_paddle_model_matrix = Matrix4::identity();
  let mut right_paddle_model_matrix = Matrix4::identity();

  let view_matrix = prelude::Matrix4::identity();
  let projection_matrix = prelude::Matrix4::orthographic(0.0, window_width as f32, window_height as f32, 0.0, -1.0, 1.0);

  let ball_start_location = prelude::Vector2::new(window_width as f32 / 2.0, window_height as f32 / 2.0);
  ball_model_matrix.translate(ball_start_location);
  
  let left_paddle_start_location = prelude::Vector2::new(32.0, window_height as f32 / 2.0);
  left_paddle_model_matrix.translate(left_paddle_start_location);

  let right_paddle_start_location = prelude::Vector2::new(window_width as f32 - 32.0, window_height as f32 / 2.0);
  right_paddle_model_matrix.translate(right_paddle_start_location);

  unsafe {
    gl::Viewport(0, 0, window_width as i32, window_height as i32);
    gl::ClearColor(0.2, 0.2, 0.4, 1.0);
    gl::UseProgram(shader_program.id());

    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
  }

  shader_program.set_view_matrix(&view_matrix)?;
  shader_program.set_projection_matrix(&projection_matrix)?;

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
    let translation_vector = Vector2::new(ball_velocity_x * deltamillis, ball_velocity_y * deltamillis);
    ball_model_matrix.translate(translation_vector);

    if ball_model_matrix.x.w >= window_width as f32 || ball_model_matrix.x.w <= 0.0 {
      ball_velocity_x *= -1.0;
    }

    if ball_model_matrix.y.w >= window_height as f32 || ball_model_matrix.y.w <= 0.0 {
      ball_velocity_y *= -1.0;
    }

    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    
    shader_program.set_model_matrix(&ball_model_matrix)?;
    ball_sprite.render();

    shader_program.set_model_matrix(&left_paddle_model_matrix)?;
    paddle_sprite.render();

    shader_program.set_model_matrix(&right_paddle_model_matrix)?;
    paddle_sprite.render();

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