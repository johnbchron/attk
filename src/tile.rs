use bevy::prelude::*;

pub enum Direction {
  North,
  East,
  South,
  West,
}

pub trait TileType {
  fn size(&self) -> Vec2;
  fn indices(&self) -> Vec<usize>;
  fn atlas_handle(&self, atlases: &TileAtlases) -> Handle<TextureAtlas>;
}

pub struct Tile<Ty: TileType> {
  pub _type:   Ty,
  pub variant: u8,
}

impl<Ty: TileType> Tile<Ty> {
  pub fn index(&self) -> usize { self._type.indices()[self.variant as usize] }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TilePosition(pub i64, pub i64);

impl TilePosition {
  fn world_position(&self) -> Vec3 {
    Vec3::new(self.0 as f32, self.1 as f32, 0.0)
  }
  pub fn transform<Ty: TileType>(&self, type_: &Ty) -> Transform {
    Transform::from_translation(self.world_position())
      .with_scale(type_.size().extend(1.0).recip())
  }
}

#[derive(Resource, Clone)]
pub struct TileAtlases {
  pub grass: Handle<TextureAtlas>,
  pub wall:  Handle<TextureAtlas>,
}

impl FromWorld for TileAtlases {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.get_resource::<AssetServer>().unwrap();

    let grass_texture_handle = asset_server.load("textures/tiles/grass.png");
    let wall_texture_handle = asset_server.load("textures/tiles/wall.png");

    let mut texture_atlases =
      world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();

    let grass_atlas = TextureAtlas::from_grid(
      grass_texture_handle,
      Vec2::new(16.0, 16.0),
      16,
      16,
      None,
      None,
    );
    let grass_atlas_handle = texture_atlases.add(grass_atlas);

    let wall_atlas = TextureAtlas::from_grid(
      wall_texture_handle,
      Vec2::new(16.0, 16.0),
      14,
      10,
      Some(Vec2::new(16.0, 16.0)),
      None,
    );
    let wall_atlas_handle = texture_atlases.add(wall_atlas);

    let atlases = TileAtlases {
      grass: grass_atlas_handle,
      wall:  wall_atlas_handle,
    };

    atlases
  }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
  fn build(&self, app: &mut App) { app.init_resource::<TileAtlases>(); }
}
