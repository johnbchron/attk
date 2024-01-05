use bevy::{prelude::*, utils::HashMap};

use crate::tile::{Direction, Tile, TileAtlases, TilePosition, TileType};

enum MapTile {
  Grass,
  FloweryGrass,
  Flagstone,
  TallWall(Direction),
}

impl TileType for MapTile {
  fn size(&self) -> Vec2 {
    match self {
      MapTile::Grass | MapTile::FloweryGrass | MapTile::Flagstone => {
        Vec2::splat(16.0)
      }
      MapTile::TallWall(_) => Vec2::splat(32.0),
    }
  }
  fn indices(&self) -> Vec<usize> {
    match self {
      MapTile::Grass => vec![2],
      MapTile::FloweryGrass => vec![29],
      MapTile::Flagstone => vec![128],
      MapTile::TallWall(direction) => match direction {
        Direction::North => vec![19],
        Direction::East => vec![16],
        Direction::South => vec![28],
        Direction::West => vec![41],
      },
    }
  }
  fn atlas_handle(&self, atlases: &TileAtlases) -> Handle<TextureAtlas> {
    match self {
      MapTile::Grass | MapTile::FloweryGrass | MapTile::Flagstone => {
        atlases.grass.clone()
      }
      MapTile::TallWall(_) => atlases.wall.clone(),
    }
  }
}

impl Tile<MapTile> {
  fn passable(&self) -> bool {
    match self._type {
      MapTile::Grass => true,
      MapTile::FloweryGrass => true,
      MapTile::Flagstone => true,
      MapTile::TallWall(_) => false,
    }
  }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) { app.add_systems(Startup, setup); }
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  atlases: Res<TileAtlases>,
) {
  commands.spawn(Camera2dBundle {
    projection: OrthographicProjection {
      near: -1000.0,
      scale: 64.0_f32.recip(),
      ..default()
    },
    ..default()
  });

  let mut map = HashMap::new();
  for i in -10..=10 {
    for j in -10..=10 {
      let pos = TilePosition(i, j);
      let _type = if i == 0 && j == 0 {
        MapTile::Flagstone
      } else if i % 2 == 0 && j % 2 == 0 {
        MapTile::FloweryGrass
      } else {
        MapTile::Grass
      };
      map.insert(pos, Tile { _type, variant: 0 });
    }
  }

  for (pos, tile) in map {
    commands.spawn(SpriteSheetBundle {
      texture_atlas: tile._type.atlas_handle(&atlases),
      transform: pos.transform(&tile._type),
      sprite: TextureAtlasSprite::new(tile.index()),
      ..Default::default()
    });
  }
}
