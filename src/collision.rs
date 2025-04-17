use crate::prelude::CollisionDirection;

pub struct Collision {
  entry_direction: CollisionDirection,
  penetration_depth: f32
}

impl Collision {
  pub fn new(entry_direction: CollisionDirection, penetration_depth: f32) -> Self {
    Self {
      entry_direction,
      penetration_depth
    }
  }

  pub fn entry_direction(&self) -> CollisionDirection {
    self.entry_direction
  }

  pub fn penetration_depth(&self) -> f32 {
    self.penetration_depth
  }
}

#[cfg(test)]
mod tests {
  use super::Collision;
  use crate::prelude::CollisionDirection;

  #[test]
  fn constructor() {
    let direction = CollisionDirection::Left;
    let depth = 4.0;

    let collision = Collision::new(direction, depth);

    assert_eq!(collision.entry_direction, direction);
    assert_eq!(collision.penetration_depth, depth);
  }
}