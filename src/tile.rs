use bevy::prelude::*;

#[derive(Clone, Copy, Reflect)]
pub enum Direction4 {
  North,
  East,
  South,
  West,
}

#[derive(Clone, Copy, Reflect)]
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

#[derive(Clone, Copy, Reflect)]
pub enum VerticalPart {
  Top,
  Bottom,
}

pub trait TileType: Reflect {
  /// The number of pixels in this tile that equals one world unit, and the the
  /// offset in world units required to center this tile.
  fn size_and_center(&self) -> (Vec2, Vec2);
  fn coords(&self) -> Vec<TileSheetCoords>;
  fn atlas_handle(&self, atlases: &TileAtlases) -> TextureAtlasWithGrid;
}

pub fn rect_range(
  x: usize,
  y: usize,
  w: usize,
  h: usize,
) -> Vec<TileSheetCoords> {
  (x..x + w)
    .flat_map(|i| (y..y + h).map(move |j| TileSheetCoords::new(i, j)))
    .collect()
}

pub fn rect_range_with_x_flip(
  x: usize,
  y: usize,
  w: usize,
  h: usize,
) -> Vec<TileSheetCoords> {
  rect_range(x, y, w, h)
    .into_iter()
    .flat_map(|coords| vec![coords.clone(), coords.flip_x()])
    .collect()
}

pub struct Tile<Ty: TileType> {
  pub _type:   Ty,
  pub variant: usize,
}

impl<Ty: TileType> Tile<Ty> {
  pub fn new(_type: Ty) -> Self { Self { _type, variant: 0 } }
  pub fn coords(&self) -> TileSheetCoords {
    let all_variants = self._type.coords();
    all_variants[self.variant % all_variants.len()].clone()
  }
  pub fn texture_atlas_sprite(
    &self,
    atlases: &TileAtlases,
  ) -> TextureAtlasSprite {
    let coords = self.coords();
    let atlas = self._type.atlas_handle(atlases);
    atlas.texture_atlas_sprite(coords)
  }
}

#[derive(Clone)]
pub struct TileSheetCoords {
  x:      usize,
  y:      usize,
  flip_x: bool,
  flip_y: bool,
}

impl TileSheetCoords {
  pub fn new(x: usize, y: usize) -> Self {
    Self {
      x,
      y,
      flip_x: false,
      flip_y: false,
    }
  }
  pub fn flip_x(mut self) -> Self {
    self.flip_x = !self.flip_x;
    self
  }
  pub fn flip_y(mut self) -> Self {
    self.flip_y = !self.flip_y;
    self
  }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TilePosition {
  pub x:     i64,
  pub y:     i64,
  pub layer: u8,
}

impl TilePosition {
  pub fn transform<Ty: TileType>(&self, type_: &Ty) -> Transform {
    let (size, offset) = type_.size_and_center();
    Transform::from_translation(
      Vec3::new(self.x as f32, self.y as f32, self.layer as f32)
        + offset.extend(0.0),
    )
    .with_scale(size.extend(1.0).recip())
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
  pub fn texture_atlas_sprite(
    &self,
    coords: TileSheetCoords,
  ) -> TextureAtlasSprite {
    TextureAtlasSprite {
      index: self.index((coords.x, coords.y)),
      flip_x: coords.flip_x,
      flip_y: coords.flip_y,
      ..Default::default()
    }
  }
}

#[derive(Resource, Clone)]
pub struct TileAtlases {
  pub grass:       TextureAtlasWithGrid,
  pub wall:        TextureAtlasWithGrid,
  pub player_base: TextureAtlasWithGrid,
}

impl FromWorld for TileAtlases {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.get_resource::<AssetServer>().unwrap();

    let grass_texture_handle = asset_server.load("textures/tiles/grass.png");
    let wall_texture_handle = asset_server.load("textures/tiles/wall.png");
    let player_base_texture_handle =
      asset_server.load("textures/player/fbas_1body_human_00.png");

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

    let player_base_atlas = TextureAtlas::from_grid(
      player_base_texture_handle,
      Vec2::new(64.0, 64.0),
      16,
      16,
      None,
      None,
    );
    let player_base_atlas_handle = texture_atlases.add(player_base_atlas);
    let player_base_atlas = TextureAtlasWithGrid {
      atlas: player_base_atlas_handle,
      grid:  (16, 16),
    };

    let atlases = TileAtlases {
      grass:       grass_atlas,
      wall:        wall_atlas,
      player_base: player_base_atlas,
    };

    atlases
  }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
  fn build(&self, app: &mut App) { app.init_resource::<TileAtlases>(); }
}
