pub mod player;
pub mod tiles;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins.set(ImagePlugin::default_nearest()),
      WorldInspectorPlugin::default(),
      tiles::TilePlugin,
      player::PlayerPlugin,
    ))
    .run();
}
