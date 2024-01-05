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
