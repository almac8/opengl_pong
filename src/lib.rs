use std::{ffi::CString, path::Path, time::Instant};
use sdl2::{event::Event, keyboard::Keycode};

mod math;
mod vertex_data;
mod buffer_object;
mod vertex_array;
mod shader;
mod shader_program;
mod texture;
mod sprite;
mod location;
mod collider;
mod collision;
mod collision_direction;
mod collision_system;
mod frame_limiter;

mod prelude {
  pub const WINDOW_WIDTH: u32 = 800;
  pub const WINDOW_HEIGHT: u32 = 600;

  pub use crate::math::Vector2;
  pub use crate::math::Vector4;
  pub use crate::math::Matrix4;
  pub use crate::vertex_data::generate_textured_vertex_data;
  pub use crate::buffer_object::BufferObject;
  pub use crate::vertex_array::VertexArray;
  pub use crate::shader::Shader;
  pub use crate::shader_program::{ShaderProgram, set_model_matrix, set_view_matrix, set_projection_matrix};
  pub use crate::texture::Texture;
  pub use crate::sprite::Sprite;
  pub use crate::location::Location;
  pub use crate::collider::Collider;
  pub use crate::collision::Collision;
  pub use crate::collision_direction::CollisionDirection;
  pub use crate::collision_system::find_collisions;
  pub use crate::frame_limiter::limit_frame_rate;
}

use prelude::{
  WINDOW_WIDTH,
  WINDOW_HEIGHT,
  find_collisions,
  limit_frame_rate,
  set_model_matrix,
  set_view_matrix,
  set_projection_matrix,
  Collider,
  CollisionDirection,
  Location,
  Shader,
  ShaderProgram,
  Sprite,
  Vector2,
  Matrix4
};

pub fn launch() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let mut event_pump = sdl_context.event_pump()?;

  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 3);

  let window = video_subsystem
    .window("Pong", WINDOW_WIDTH, WINDOW_HEIGHT)
    .opengl()
    .build()
    .map_err(|error| error.to_string())?;

  let _gl_context = window.gl_create_context()?;
  gl::load_with(|procname| video_subsystem.gl_get_proc_address(procname) as *const gl::types::GLvoid);

  let mut is_running = true;

  let power_up_sprite = Sprite::new(Path::new("res/textures/power_up.png"), 16, 16)?;
  let ball_sprite = Sprite::new(Path::new("res/textures/ball.png"), 16, 16)?;
  let paddle_sprite = Sprite::new(Path::new("res/textures/paddle.png"), 16, 128)?;

  let shader_program = ShaderProgram::link(
    vec![
      Shader::vertex(Path::new("res/shaders/vertex_shader.glsl"))?,
      Shader::fragment(Path::new("res/shaders/fragment_shader.glsl"))?
    ]
  );
  
  let mut power_up_locations = vec![
    Location::new(WINDOW_WIDTH as f32 / 4.0, (WINDOW_HEIGHT as f32 / 4.0) * 3.0),
    Location::new((WINDOW_WIDTH as f32 / 4.0) * 3.0, (WINDOW_HEIGHT as f32 / 4.0) * 3.0),
    Location::new((WINDOW_WIDTH as f32 / 4.0) * 3.0, WINDOW_HEIGHT as f32 / 4.0),
    Location::new(WINDOW_WIDTH as f32 / 4.0, WINDOW_HEIGHT as f32 / 4.0)
  ];

  let mut ball_location = Location::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0);
  let mut left_paddle_location = Location::new(32.0, WINDOW_HEIGHT as f32 / 2.0);
  let mut right_paddle_location = Location::new(WINDOW_WIDTH as f32 - 32.0, WINDOW_HEIGHT as f32 / 2.0);

  let view_matrix = Matrix4::identity();
  let projection_matrix = Matrix4::orthographic(0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32, 0.0, -1.0, 1.0);

  unsafe {
    gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
    gl::ClearColor(0.2, 0.2, 0.4, 1.0);
    gl::UseProgram(shader_program.id());

    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
  }

  set_view_matrix(&shader_program, &view_matrix)?;
  set_projection_matrix(&shader_program, &projection_matrix)?;

  let ball_speed = 0.5;
  let mut ball_velocity = Vector2::new(0.5, 0.5);
  let mut left_paddle_velocity = Vector2::new(0.0, 0.0);
  let mut right_paddle_velocity = Vector2::new(0.0, 0.0);
  
  let barrier_thickness = 8.0;
  
  let mut colliders = vec![];

  let mut power_up_collider_indices = vec![];
  for power_up_location in &power_up_locations {
    power_up_collider_indices.push(colliders.len());
    colliders.push(Collider::new(power_up_location.x(), power_up_location.y(), 16.0, 16.0));
  }

  let ball_collider_index = colliders.len();
  colliders.push(Collider::new(ball_location.x(), ball_location.y(), 16.0, 16.0));

  let left_paddle_collider_index = colliders.len();
  colliders.push(Collider::new(left_paddle_location.x(), left_paddle_location.y(), 16.0, 128.0));

  let right_paddle_collider_index = colliders.len();
  colliders.push(Collider::new(right_paddle_location.x(), right_paddle_location.y(), 16.0, 128.0));
  
  let left_barrier_collider_index = colliders.len();
  colliders.push(Collider::new(0.0, (WINDOW_HEIGHT / 2) as f32, barrier_thickness, WINDOW_HEIGHT as f32));
  
  let right_barrier_collider_index = colliders.len();
  colliders.push(Collider::new(WINDOW_WIDTH as f32, (WINDOW_HEIGHT / 2) as f32, barrier_thickness, WINDOW_HEIGHT as f32));
  
  let top_barrier_collider_index = colliders.len();
  colliders.push(Collider::new((WINDOW_WIDTH / 2) as f32, 0.0, WINDOW_WIDTH as f32, barrier_thickness));
  
  let bottom_barrier_collider_index = colliders.len();
  colliders.push(Collider::new((WINDOW_WIDTH / 2) as f32, WINDOW_HEIGHT as f32, WINDOW_WIDTH as f32, barrier_thickness));

  let mut current_time = Instant::now();
  let mut previous_time = current_time;
  while is_running {
    current_time = Instant::now();
    let deltatime = current_time - previous_time;
    previous_time = current_time;
    let deltamillis = deltatime.as_millis() as f32;

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => is_running = false,

        Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
          if !repeat {
            match keycode {
              Keycode::W => left_paddle_velocity.y += -0.5,
              Keycode::S => left_paddle_velocity.y += 0.5,
  
              _ => {}
            }
          }
        },

        Event::KeyUp { keycode: Some(keycode), .. } => {
          match keycode {
            Keycode::W => left_paddle_velocity.y -= -0.5,
            Keycode::S => left_paddle_velocity.y -= 0.5,

            _ => {}
          }
        },

        _ => {}
      }
    }
    
    if ball_velocity.x < 0.0 {
      right_paddle_velocity.y = 0.0;
    } else {
      if right_paddle_location.y() < ball_location.y() {
        right_paddle_velocity.y = 0.5;
      }

      if right_paddle_location.y() > ball_location.y() {
        right_paddle_velocity.y = -0.5;
      }
    }

    let ball_translation = ball_velocity.normalized() * ball_speed * deltamillis;
    ball_location.translate(ball_translation);

    let left_paddle_translation = left_paddle_velocity * deltamillis;
    left_paddle_location.translate(left_paddle_translation);

    let right_paddle_translation = right_paddle_velocity * deltamillis;
    right_paddle_location.translate(right_paddle_translation);

    colliders[ball_collider_index].set_location(&ball_location);
    colliders[left_paddle_collider_index].set_location(&left_paddle_location);
    colliders[right_paddle_collider_index].set_location(&right_paddle_location);

    let collisions = find_collisions(&colliders);
    if let Some(collisions) = collisions {
      for collision in collisions {
        if collision.primary_index() == ball_collider_index {
          if collision.secondary_index() == left_barrier_collider_index {
            ball_location.translate(Vector2::new(collision.penetration_depth(), 0.0));
            ball_velocity.x *= -1.0;
          }
          
          if collision.secondary_index() == right_barrier_collider_index {
            ball_location.translate(Vector2::new(-collision.penetration_depth(), 0.0));
            ball_velocity.x *= -1.0;
          }
          
          if collision.secondary_index() == top_barrier_collider_index {
            ball_location.translate(Vector2::new(0.0, collision.penetration_depth()));
            ball_velocity.y *= -1.0;
          }
          
          if collision.secondary_index() == bottom_barrier_collider_index {
            ball_location.translate(Vector2::new(0.0, -collision.penetration_depth()));
            ball_velocity.y *= -1.0;
          }

          if collision.secondary_index() == left_paddle_collider_index {
            match collision.entry_direction() {
              CollisionDirection::Left => {
                ball_location.translate(Vector2::new(-collision.penetration_depth(), 0.0));
                ball_velocity.x *= -1.0;
              },

              CollisionDirection::Right => {
                ball_location.translate(Vector2::new(collision.penetration_depth(), 0.0));

                let signed_offset = left_paddle_location.y() - ball_location.y();
                let unsigned_offset = if signed_offset > 0.0 { signed_offset } else { signed_offset * -1.0 };
                let ratio = unsigned_offset / 64.0;

                let final_y = ratio / 2.0;
                let final_x = 0.5 - final_y;

                ball_velocity.x = final_x;
                ball_velocity.y = if signed_offset > 0.0 { final_y * -1.0 } else { final_y };
              },

              CollisionDirection::Top => {
                ball_location.translate(Vector2::new(0.0, -collision.penetration_depth()));
                ball_velocity.y *= -1.0;
                ball_velocity.x = 0.5;
              },

              CollisionDirection::Bottom => {
                ball_location.translate(Vector2::new(0.0, collision.penetration_depth()));
                ball_velocity.y *= -1.0;
                ball_velocity.x = 0.5;
              }
            }
          }

          if collision.secondary_index() == right_paddle_collider_index {
            match collision.entry_direction() {
              CollisionDirection::Left => {
                ball_location.translate(Vector2::new(-collision.penetration_depth(), 0.0));
                ball_velocity.x *= -1.0;
              },

              CollisionDirection::Right => {
                ball_location.translate(Vector2::new(collision.penetration_depth(), 0.0));
                ball_velocity.x *= -1.0;
              },

              CollisionDirection::Top => {
                ball_location.translate(Vector2::new(0.0, -collision.penetration_depth()));
                ball_velocity.y *= -1.0;
              },

              CollisionDirection::Bottom => {
                ball_location.translate(Vector2::new(0.0, collision.penetration_depth()));
                ball_velocity.y *= -1.0;
              }
            }
          }

          for power_up_collider_index in &power_up_collider_indices {
            if collision.secondary_index() == *power_up_collider_index {
              let index = collision.secondary_index();

              power_up_locations[index].translate(
                Vector2::new(-(WINDOW_WIDTH as f32), -(WINDOW_HEIGHT as f32))
              );

              colliders[index].set_location(&power_up_locations[index]);
            }
          }
        }

        if collision.primary_index() == left_paddle_collider_index {
          if collision.secondary_index() == top_barrier_collider_index {
            left_paddle_location.translate(Vector2::new(0.0, collision.penetration_depth()));
          }

          if collision.secondary_index() == bottom_barrier_collider_index {
            left_paddle_location.translate(Vector2::new(0.0, -collision.penetration_depth()));
          }
        }

        if collision.primary_index() == right_paddle_collider_index {
          if collision.secondary_index() == top_barrier_collider_index {
            right_paddle_location.translate(Vector2::new(0.0, collision.penetration_depth()));
          }

          if collision.secondary_index() == bottom_barrier_collider_index {
            right_paddle_location.translate(Vector2::new(0.0, -collision.penetration_depth()));
          }
        }
      }
    }

    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    for power_up_location in &power_up_locations {
      set_model_matrix(&shader_program, &power_up_location.matrix())?;
      power_up_sprite.render();
    }

    set_model_matrix(&shader_program, &ball_location.matrix())?;
    ball_sprite.render();
    
    set_model_matrix(&shader_program, &left_paddle_location.matrix())?;
    paddle_sprite.render();
    
    set_model_matrix(&shader_program, &right_paddle_location.matrix())?;
    paddle_sprite.render();

    window.gl_swap_window();
    
    limit_frame_rate(previous_time, 60);
  }

  Ok(())
}