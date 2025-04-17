use crate::prelude::CollisionDirection;

pub struct Collision {
  primary_index: usize,
  secondary_index: usize,
  entry_direction: CollisionDirection,
  penetration_depth: f32
}

impl Collision {
  pub fn new(primary_index: usize, secondary_index: usize, entry_direction: CollisionDirection, penetration_depth: f32) -> Self {
    Self {
      primary_index,
      secondary_index,
      entry_direction,
      penetration_depth
    }
  }

  pub fn primary_index(&self) -> usize {
    self.primary_index
  }

  pub fn secondary_index(&self) -> usize {
    self.secondary_index
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

    let collision = Collision::new(0, 0, direction, depth);

    assert_eq!(collision.entry_direction, direction);
    assert_eq!(collision.penetration_depth, depth);
  }
}