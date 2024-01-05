use bevy::prelude::*;

use crate::tile::{
  Direction4, TextureAtlasWithGrid, Tile, TileAtlases, TilePosition,
  TileSheetCoords, TileType,
};

enum PlayerSprite {
  Stand(Direction4),
}

impl TileType for PlayerSprite {
  fn size(&self) -> Vec2 { Vec2::splat(32.0) }
  fn coords(&self) -> Vec<TileSheetCoords> {
    match self {
      PlayerSprite::Stand(Direction4::North) => {
        vec![TileSheetCoords::new(1, 0)]
      }
      PlayerSprite::Stand(Direction4::East) => {
        vec![TileSheetCoords::new(2, 0).flip_x()]
      }
      PlayerSprite::Stand(Direction4::South) => {
        vec![TileSheetCoords::new(0, 0)]
      }
      PlayerSprite::Stand(Direction4::West) => vec![TileSheetCoords::new(2, 0)],
    }
  }
  fn atlas_handle(&self, atlases: &TileAtlases) -> TextureAtlasWithGrid {
    atlases.player_base.clone()
  }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) { app.add_systems(Startup, setup); }
}

fn setup(mut commands: Commands, atlases: Res<TileAtlases>) {
  let tile = Tile {
    _type:   PlayerSprite::Stand(Direction4::South),
    variant: 0,
  };
  let pos = TilePosition {
    x:     0,
    y:     0,
    layer: 0,
  };

  commands.spawn((
    SpriteSheetBundle {
      texture_atlas: tile._type.atlas_handle(&atlases).atlas,
      transform: pos.transform(&tile._type),
      sprite: tile.texture_atlas_sprite(&atlases),
      ..Default::default()
    },
    Name::new("player"),
  ));
}
