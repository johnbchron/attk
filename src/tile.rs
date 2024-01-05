use bevy::prelude::*;

pub enum Direction4 {
  North,
  East,
  South,
  West,
}

pub enum Direction8 {
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest,
}

pub enum VerticalPart {
  Top,
  Bottom,
}

pub trait TileType {
  fn size(&self) -> Vec2;
  fn coords(&self) -> Vec<(usize, usize)>;
  fn atlas_handle(&self, atlases: &TileAtlases) -> TextureAtlasWithGrid;
}

pub fn rect_range(
  x: usize,
  y: usize,
  w: usize,
  h: usize,
) -> Vec<(usize, usize)> {
  (x..x + w)
    .flat_map(|i| (y..y + h).map(move |j| (i, j)))
    .collect()
}

pub struct Tile<Ty: TileType> {
  pub _type:   Ty,
  pub variant: usize,
}

impl<Ty: TileType> Tile<Ty> {
  pub fn coords(&self) -> (usize, usize) {
    let all_variants = self._type.coords();
    all_variants[self.variant % all_variants.len()]
  }
  pub fn index(&self, atlases: &TileAtlases) -> usize {
    self._type.atlas_handle(atlases).index(self.coords())
  }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TilePosition {
  pub x:     i64,
  pub y:     i64,
  pub layer: u8,
}

impl TilePosition {
  fn world_position(&self) -> Vec3 {
    Vec3::new(self.x as f32, self.y as f32, self.layer as f32)
  }
  pub fn transform<Ty: TileType>(&self, type_: &Ty) -> Transform {
    Transform::from_translation(self.world_position())
      .with_scale(type_.size().extend(1.0).recip())
  }
}

#[derive(Clone)]
pub struct TextureAtlasWithGrid {
  pub atlas: Handle<TextureAtlas>,
  grid:      (usize, usize),
}

impl TextureAtlasWithGrid {
  pub fn index(&self, coords: (usize, usize)) -> usize {
    coords.0 + coords.1 * self.grid.0
  }
}

#[derive(Resource, Clone)]
pub struct TileAtlases {
  pub grass: TextureAtlasWithGrid,
  pub wall:  TextureAtlasWithGrid,
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
      Vec2::new(32.0, 32.0),
      8,
      8,
      None,
      None,
    );
    let grass_atlas_handle = texture_atlases.add(grass_atlas);
    let grass_atlas = TextureAtlasWithGrid {
      atlas: grass_atlas_handle,
      grid:  (8, 8),
    };

    let wall_atlas = TextureAtlas::from_grid(
      wall_texture_handle,
      Vec2::new(16.0, 16.0),
      14,
      10,
      Some(Vec2::new(16.0, 16.0)),
      None,
    );
    let wall_atlas_handle = texture_atlases.add(wall_atlas);
    let wall_atlas = TextureAtlasWithGrid {
      atlas: wall_atlas_handle,
      grid:  (14, 10),
    };

    let atlases = TileAtlases {
      grass: grass_atlas,
      wall:  wall_atlas,
    };

    atlases
  }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
  fn build(&self, app: &mut App) { app.init_resource::<TileAtlases>(); }
}
