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
  Walk(Vec2),
  Run(Vec2),
}

impl Default for PlayerStatus {
  fn default() -> Self { PlayerStatus::Stand(Direction4::South) }
}

#[derive(Resource)]
pub struct PlayerSpeeds {
  pub walk: f32,
  pub run:  f32,
}

impl Default for PlayerSpeeds {
  fn default() -> Self {
    PlayerSpeeds {
      walk: 4.0,
      run:  6.0,
    }
  }
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
      PlayerStatus::Walk(_) => {
        todo!()
      }
      PlayerStatus::Run(_) => {
        todo!()
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
      .init_resource::<PlayerSpeeds>()
      .add_systems(Startup, setup)
      .add_systems(
        Update,
        (
          accept_movement_input,
          (apply_movement, update_player_sprite),
        )
          .chain(),
      );
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

fn accept_movement_input(
  keyboard_input: Res<Input<KeyCode>>,
  speeds: Res<PlayerSpeeds>,
  mut query: Query<&mut Player>,
) {
  for mut player in query.iter_mut() {
    let mut movement = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
      movement.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
      movement.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
      movement.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
      movement.x -= 1.0;
    }
    movement = movement.normalize_or_zero();

    let run = keyboard_input.pressed(KeyCode::ShiftLeft);
    if movement != Vec2::ZERO {
      if run {
        player.0 = PlayerStatus::Run(movement);
      } else {
        player.0 = PlayerStatus::Walk(movement);
      }
    } else {
      player.0 = PlayerStatus::Stand(Direction4::South);
    }
  }
}

fn apply_movement(
  mut query: Query<(&mut Transform, &Player)>,
  time: Res<Time>,
) {
  for (mut transform, player) in query.iter_mut() {
    match player.0 {
      PlayerStatus::Walk(movement) | PlayerStatus::Run(movement) => {
        transform.translation += movement.extend(0.0) * time.delta_seconds();
      }
      _ => {}
    }
  }
}
