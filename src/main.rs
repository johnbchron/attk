pub mod camera;
pub mod map;
pub mod player;
pub mod tile;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins.set(ImagePlugin::default_nearest()),
      WorldInspectorPlugin::default(),
      tile::TilePlugin,
      map::MapPlugin,
      player::PlayerPlugin,
      camera::CameraPlugin,
    ))
    .run();
}
