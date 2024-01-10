use bevy::prelude::*;

use self::status::PlayerStatus;
use crate::tile::{
  AnimatedTile, Direction4, Tile, TileAtlases, TilePosition, TileType,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player(pub PlayerStatus);

mod status;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerSpeeds {
  pub walk: f32,
  pub run:  f32,
}

impl Default for PlayerSpeeds {
  fn default() -> Self {
    PlayerSpeeds {
      walk: 3.0,
      run:  4.0,
    }
  }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Player>()
      .register_type::<Tile<PlayerStatus>>()
      .register_type::<AnimatedTile<PlayerStatus>>()
      .register_type::<PlayerSpeeds>()
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

#[allow(clippy::type_complexity)]
fn update_player_sprite(
  mut commands: Commands,
  mut query: Query<
    (
      Entity,
      &Player,
      &mut TextureAtlasSprite,
      Option<&mut AnimatedTile<PlayerStatus>>,
    ),
    Changed<Player>,
  >,
  atlases: Res<TileAtlases>,
  time: Res<Time>,
) {
  for (entity, player, mut sprite, tile) in query.iter_mut() {
    if let Some(mut tile) = tile {
      // if the status matches, just tick the tile
      if tile.tile._type == player.0 {
        tile.tick(time.delta_seconds());
      } else {
        // if the style doesn't match, reset the tick unless we're going
        // from run -> walk or walk -> run
        let run_to_walk = matches!(
          (&tile.tile._type, &player.0),
          (PlayerStatus::Walk(_), PlayerStatus::Run(_))
        );
        let walk_to_run = matches!(
          (&tile.tile._type, &player.0),
          (PlayerStatus::Run(_), PlayerStatus::Walk(_))
        );
        if run_to_walk || walk_to_run {
          let mut new_tile = AnimatedTile::new(Tile::new(player.0.clone()));
          new_tile.time = tile.time;
          *tile = new_tile;
          tile.tick(time.delta_seconds());
        } else {
          *tile = AnimatedTile::new(Tile::new(player.0.clone()));
        }
      }
      *sprite = tile.tile.texture_atlas_sprite(&atlases);
    } else {
      let tile = AnimatedTile::new(Tile::new(player.0.clone()));
      *sprite = tile.tile.texture_atlas_sprite(&atlases);
      commands.entity(entity).insert(tile);
    }
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

    let old_status = player.0.clone();
    if movement != Vec2::ZERO {
      if run {
        player.0 = PlayerStatus::Run(movement * speeds.run);
      } else {
        player.0 = PlayerStatus::Walk(movement * speeds.walk);
      }
    } else {
      player.0 = PlayerStatus::Stand(old_status.direction());
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
