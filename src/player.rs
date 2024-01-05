use bevy::prelude::*;

use crate::tile::{
  Direction4, TextureAtlasWithGrid, Tile, TileAtlases, TilePosition,
  TileSheetCoords, TileType,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player(pub PlayerStatus);

#[derive(Clone, Reflect)]
pub enum PlayerStatus {
  Stand(Direction4),
}

impl Default for PlayerStatus {
  fn default() -> Self { PlayerStatus::Stand(Direction4::South) }
}

impl TileType for PlayerStatus {
  fn size_and_center(&self) -> (Vec2, Vec2) {
    (Vec2::splat(16.0), Vec2::new(0.0, 0.5))
  }
  fn coords(&self) -> Vec<TileSheetCoords> {
    match self {
      PlayerStatus::Stand(Direction4::North) => {
        vec![TileSheetCoords::new(0, 1)]
      }
      PlayerStatus::Stand(Direction4::East) => {
        vec![TileSheetCoords::new(0, 2)]
      }
      PlayerStatus::Stand(Direction4::South) => {
        vec![TileSheetCoords::new(0, 0)]
      }
      PlayerStatus::Stand(Direction4::West) => {
        vec![TileSheetCoords::new(0, 2).flip_x()]
      }
    }
  }
  fn atlas_handle(&self, atlases: &TileAtlases) -> TextureAtlasWithGrid {
    atlases.player_base.clone()
  }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Player>()
      .add_systems(Startup, setup)
      .add_systems(Update, update_player_sprite);
  }
}

fn setup(mut commands: Commands, atlases: Res<TileAtlases>) {
  let status = PlayerStatus::Stand(Direction4::South);

  let tile = Tile::new(status.clone());
  let pos = TilePosition {
    x:     0,
    y:     0,
    layer: 1,
  };

  commands.spawn((
    SpriteSheetBundle {
      texture_atlas: tile._type.atlas_handle(&atlases).atlas,
      transform: pos.transform(&tile._type),
      sprite: tile.texture_atlas_sprite(&atlases),
      ..Default::default()
    },
    Name::new("player"),
    Player(status),
  ));
}

fn update_player_sprite(
  mut query: Query<(&Player, &mut TextureAtlasSprite), Changed<Player>>,
  atlases: Res<TileAtlases>,
) {
  for (player, mut sprite) in query.iter_mut() {
    let tile = Tile::new(player.0.clone());
    *sprite = tile.texture_atlas_sprite(&atlases);
  }
}
