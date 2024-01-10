use bevy::prelude::*;
#[derive(Clone, Copy, Reflect, PartialEq, Eq, Default)]
pub enum Direction4 {
  North,
  East,
  #[default]
  South,
  West,
}

impl TryFrom<Vec2> for Direction4 {
  type Error = ();

  fn try_from(mut value: Vec2) -> Result<Self, Self::Error> {
    value = value.normalize_or_zero();
    if value == Vec2::ZERO {
      return Err(());
    }

    // favor vertical directions if a multiple of 45 degrees
    if value.x.abs() <= 0.5 {
      if value.y > 0.0 {
        return Ok(Direction4::North);
      } else {
        return Ok(Direction4::South);
      }
    } else {
      if value.x > 0.0 {
        return Ok(Direction4::East);
      } else {
        return Ok(Direction4::West);
      }
    }
  }
}

#[derive(Clone, Copy, Reflect, PartialEq, Eq, Default)]
pub enum Direction8 {
  North,
  NorthEast,
  East,
  SouthEast,
  #[default]
  South,
  SouthWest,
  West,
  NorthWest,
}

#[derive(Clone, Copy, Reflect)]
pub enum VerticalPart {
  Top,
  Bottom,
}
