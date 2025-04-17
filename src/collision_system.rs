use crate::prelude::{Collider, Collision, CollisionDirection};

pub fn find_collision(primary: &Collider, secondary: &Collider) -> Option<Collision> {
  let primary_left = primary.location().x() - (primary.width() / 2.0);
  let primary_right = primary.location().x() + (primary.width() / 2.0);
  let primary_top = primary.location().y() - (primary.height() / 2.0);
  let primary_bottom = primary.location().y() + (primary.height() / 2.0);

  let secondary_left = secondary.location().x() - (secondary.width() / 2.0);
  let secondary_right = secondary.location().x() + (secondary.width() / 2.0);
  let secondary_top = secondary.location().y() - (secondary.height() / 2.0);
  let secondary_bottom = secondary.location().y() + (secondary.height() / 2.0);

  let left_overlap = primary_right - secondary_left;
  let right_overlap = secondary_right - primary_left;
  let top_overlap = primary_bottom - secondary_top;
  let bottom_overlap = secondary_bottom - primary_top;

  if left_overlap > 0.0
  && right_overlap > 0.0
  && top_overlap > 0.0
  && bottom_overlap > 0.0 {
    let horizontal_overlap = if left_overlap < right_overlap { left_overlap } else { right_overlap };
    let horizontal_direction = if left_overlap < right_overlap { CollisionDirection::Left } else { CollisionDirection::Right };
    
    let vertical_overlap = if top_overlap < bottom_overlap { top_overlap } else { bottom_overlap };
    let vertical_direction = if top_overlap < bottom_overlap { CollisionDirection::Top } else { CollisionDirection::Bottom };
    
    let final_overlap = if horizontal_overlap < vertical_overlap { horizontal_overlap } else { vertical_overlap };
    let final_direction = if horizontal_overlap < vertical_overlap { horizontal_direction } else { vertical_direction };

    return Some(Collision::new(final_direction, final_overlap));
  }

  None
}

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
  fn no_collision() {
    let primary_collider = Collider::new(-64.0, 0.0, 32.0, 32.0);
    let secondary_collider = Collider::new(0.0, 0.0, 64.0, 64.0);

    let collision = find_collision(&primary_collider, &secondary_collider);

    assert!(collision.is_none());
  }

  #[test]
  fn left_collision() {
    let primary_collider = Collider::new(-32.0, 0.0, 32.0, 32.0);
    let secondary_collider = Collider::new(0.0, 0.0, 64.0, 64.0);

    let collision = find_collision(&primary_collider, &secondary_collider);
    
    match collision {
      Some(collision) => {
        assert_eq!(collision.entry_direction(), CollisionDirection::Left);
        assert_eq!(collision.penetration_depth(), 16.0);
      },

      None => panic!("No Collision Detected")
    }
  }

  #[test]
  fn right_collision() {
    let primary_collider = Collider::new(32.0, 0.0, 32.0, 32.0);
    let secondary_collider = Collider::new(0.0, 0.0, 64.0, 64.0);

    let collision = find_collision(&primary_collider, &secondary_collider);
    
    match collision {
      Some(collision) => {
        assert_eq!(collision.entry_direction(), CollisionDirection::Right);
        assert_eq!(collision.penetration_depth(), 16.0);
      },

      None => panic!("No Collision Detected")
    }
  }

  #[test]
  fn top_collision() {
    let primary_collider = Collider::new(0.0, -32.0, 32.0, 32.0);
    let secondary_collider = Collider::new(0.0, 0.0, 64.0, 64.0);

    let collision = find_collision(&primary_collider, &secondary_collider);
    
    match collision {
      Some(collision) => {
        assert_eq!(collision.entry_direction(), CollisionDirection::Top);
        assert_eq!(collision.penetration_depth(), 16.0);
      },

      None => panic!("No Collision Detected")
    }
  }

  #[test]
  fn bottom_collision() {
    let primary_collider = Collider::new(0.0, 32.0, 32.0, 32.0);
    let secondary_collider = Collider::new(0.0, 0.0, 64.0, 64.0);

    let collision = find_collision(&primary_collider, &secondary_collider);
    
    match collision {
      Some(collision) => {
        assert_eq!(collision.entry_direction(), CollisionDirection::Bottom);
        assert_eq!(collision.penetration_depth(), 16.0);
      },

      None => panic!("No Collision Detected")
    }
  }
}