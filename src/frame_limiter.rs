use std::time::{Duration, Instant};

pub fn limit_frame_rate(frame_start_time: Instant, target_fps: u32) {
  let end_time = Instant::now();
  let frame_duration = end_time - frame_start_time;
  let frame_max_duration = Duration::from_secs(1) / target_fps;

  if frame_duration.as_millis() < frame_max_duration.as_millis() {
    let time_left = frame_max_duration - frame_duration;
    std::thread::sleep(time_left);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn frame_limiting() {
    let target_fps = 60;
    let start_time = Instant::now();
    limit_frame_rate(start_time, target_fps);
    let end_time = Instant::now();
    let frame_duration = end_time - start_time;
    let frame_millis = frame_duration.as_millis();
    
    assert_eq!(frame_millis, (1000 / target_fps) as u128);
  }
}