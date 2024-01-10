use bevy::prelude::*;

use super::player::Player;

pub struct CameraPlugin;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MainCamera;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CameraPlayerOffset(pub Vec2);

impl Default for CameraPlayerOffset {
  fn default() -> Self { CameraPlayerOffset(Vec2::new(0.0, 0.25)) }
}

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<MainCamera>()
      .register_type::<CameraPlayerOffset>()
      .init_resource::<CameraPlayerOffset>()
      .add_systems(Startup, setup)
      .add_systems(Update, follow_player.after(crate::player::apply_movement));
  }
}

fn setup(mut commands: Commands) {
  commands.spawn((
    Camera2dBundle {
      projection: OrthographicProjection {
        near: -1000.0,
        scale: 64.0_f32.recip(),
        ..default()
      },
      ..default()
    },
    MainCamera,
  ));
}

fn follow_player(
  mut query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
  player: Query<&Transform, With<Player>>,
  offset: Res<CameraPlayerOffset>,
) {
  if let Some(mut transform) = query.iter_mut().next() {
    let player_transform = player.single();
    transform.translation =
      (player_transform.translation.xy() - offset.0).extend(0.0);
  }
}
