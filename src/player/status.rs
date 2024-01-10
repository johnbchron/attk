use bevy::prelude::*;

use crate::tile::{
  Direction4, TextureAtlasWithGrid, TileAtlases, TileSheetCoords, TileType,
};

#[derive(Clone, Reflect, PartialEq)]
pub enum PlayerStatus {
  Stand(Direction4),
  Walk(Vec2),
  Run(Vec2),
}

impl Default for PlayerStatus {
  fn default() -> Self { PlayerStatus::Stand(Direction4::South) }
}

impl PlayerStatus {
  pub fn direction(&self) -> Direction4 {
    match self {
      PlayerStatus::Stand(dir) => *dir,
      PlayerStatus::Walk(dir) => {
        Direction4::try_from(*dir).unwrap_or(Direction4::South)
      }
      PlayerStatus::Run(dir) => {
        Direction4::try_from(*dir).unwrap_or(Direction4::South)
      }
    }
  }
}
impl TileType for PlayerStatus {
  fn size_and_center(&self) -> (Vec2, Vec2) {
    (Vec2::splat(16.0), Vec2::new(0.0, 0.5))
  }
  fn coords(&self) -> Vec<TileSheetCoords> {
    match self {
      PlayerStatus::Stand(dir) => match dir {
        Direction4::North => {
          vec![TileSheetCoords::new(0, 1)]
        }
        Direction4::East => {
          vec![TileSheetCoords::new(0, 2)]
        }
        Direction4::South => {
          vec![TileSheetCoords::new(0, 0)]
        }
        Direction4::West => {
          vec![TileSheetCoords::new(0, 2).flip_x()]
        }
      },
      PlayerStatus::Walk(dir) => {
        let Ok(dir) = Direction4::try_from(*dir) else {
          return vec![TileSheetCoords::new(0, 0)];
        };
        match dir {
          Direction4::North => {
            vec![
              TileSheetCoords::new(4, 3),
              TileSheetCoords::new(5, 3),
              TileSheetCoords::new(6, 3),
              TileSheetCoords::new(4, 3).flip_x(),
              TileSheetCoords::new(5, 3).flip_x(),
              TileSheetCoords::new(6, 3).flip_x(),
            ]
          }
          Direction4::East => {
            vec![
              TileSheetCoords::new(0, 4),
              TileSheetCoords::new(1, 4),
              TileSheetCoords::new(2, 4),
              TileSheetCoords::new(3, 4),
              TileSheetCoords::new(4, 4),
              TileSheetCoords::new(5, 4),
            ]
          }
          Direction4::South => {
            vec![
              TileSheetCoords::new(0, 3),
              TileSheetCoords::new(1, 3),
              TileSheetCoords::new(2, 3),
              TileSheetCoords::new(0, 3).flip_x(),
              TileSheetCoords::new(1, 3).flip_x(),
              TileSheetCoords::new(2, 3).flip_x(),
            ]
          }
          Direction4::West => {
            vec![
              TileSheetCoords::new(0, 4).flip_x(),
              TileSheetCoords::new(1, 4).flip_x(),
              TileSheetCoords::new(2, 4).flip_x(),
              TileSheetCoords::new(3, 4).flip_x(),
              TileSheetCoords::new(4, 4).flip_x(),
              TileSheetCoords::new(5, 4).flip_x(),
            ]
          }
        }
      }
      PlayerStatus::Run(dir) => {
        let Ok(dir) = Direction4::try_from(*dir) else {
          return vec![TileSheetCoords::new(0, 0)];
        };
        match dir {
          Direction4::North => {
            vec![
              TileSheetCoords::new(4, 3),
              TileSheetCoords::new(5, 3),
              TileSheetCoords::new(7, 3),
              TileSheetCoords::new(4, 3).flip_x(),
              TileSheetCoords::new(5, 3).flip_x(),
              TileSheetCoords::new(7, 3).flip_x(),
            ]
          }
          Direction4::East => {
            vec![
              TileSheetCoords::new(0, 4),
              TileSheetCoords::new(1, 4),
              TileSheetCoords::new(6, 4),
              TileSheetCoords::new(3, 4),
              TileSheetCoords::new(4, 4),
              TileSheetCoords::new(7, 4),
            ]
          }
          Direction4::South => {
            vec![
              TileSheetCoords::new(0, 3),
              TileSheetCoords::new(1, 3),
              TileSheetCoords::new(3, 3),
              TileSheetCoords::new(0, 3).flip_x(),
              TileSheetCoords::new(1, 3).flip_x(),
              TileSheetCoords::new(3, 3).flip_x(),
            ]
          }
          Direction4::West => {
            vec![
              TileSheetCoords::new(0, 4).flip_x(),
              TileSheetCoords::new(1, 4).flip_x(),
              TileSheetCoords::new(6, 4).flip_x(),
              TileSheetCoords::new(3, 4).flip_x(),
              TileSheetCoords::new(4, 4).flip_x(),
              TileSheetCoords::new(7, 4).flip_x(),
            ]
          }
        }
      }
    }
  }
  fn atlas_handle(&self, atlases: &TileAtlases) -> TextureAtlasWithGrid {
    atlases.player_base.clone()
  }
  fn anim_speed(&self) -> Option<f32> {
    match self {
      PlayerStatus::Walk(_) => Some(8.0),
      PlayerStatus::Run(_) => Some(8.0),
      _ => None,
    }
  }
}
