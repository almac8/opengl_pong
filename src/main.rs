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

  unsafe {
    gl::Viewport(0, 0, window_width as i32, window_height as i32);
    gl::ClearColor(1.0, 0.5, 1.0, 1.0);
  }

  while is_running {
    for event in event_pump.poll_iter() {
      if let Event::Quit { .. } = event {
        is_running = false;
      }
    }

    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

    window.gl_swap_window();
  }

  Ok(())
}