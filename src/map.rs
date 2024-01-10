use bevy::{prelude::*, utils::HashMap};

use crate::tile::{
  rect_range_with_x_flip, Direction8, TextureAtlasWithGrid, Tile, TileAtlases,
  TilePosition, TileSheetCoords, TileType, VerticalPart,
};

#[derive(Clone, Reflect)]
enum MapTile {
  Grass,
  FloweryGrass,
  Flagstone,
  TallWall {
    corner: Direction8,
    height: VerticalPart,
  },
}

impl TileType for MapTile {
  fn size_and_center(&self) -> (Vec2, Vec2) { (Vec2::splat(16.0), Vec2::ZERO) }
  fn coords(&self) -> Vec<TileSheetCoords> {
    match self {
      MapTile::Grass => rect_range_with_x_flip(0, 0, 4, 4),
      MapTile::FloweryGrass => rect_range_with_x_flip(4, 0, 4, 4),
      MapTile::Flagstone => rect_range_with_x_flip(0, 4, 2, 3),
      MapTile::TallWall { .. } => {
        todo!()
      }
    }
  }
  fn atlas_handle(&self, atlases: &TileAtlases) -> TextureAtlasWithGrid {
    match self {
      MapTile::Grass | MapTile::FloweryGrass | MapTile::Flagstone => {
        atlases.grass.clone()
      }
      MapTile::TallWall { .. } => atlases.wall.clone(),
    }
  }
}

impl Tile<MapTile> {
  #[allow(dead_code)]
  fn passable(&self) -> bool {
    match self._type {
      MapTile::Grass => true,
      MapTile::FloweryGrass => true,
      MapTile::Flagstone => true,
      MapTile::TallWall { .. } => false,
    }
  }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) { app.add_systems(Startup, setup); }
}

fn setup(mut commands: Commands, atlases: Res<TileAtlases>) {
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
      let pos = TilePosition {
        x:     i * 2,
        y:     j * 2,
        layer: 0,
      };
      let _type = if i == 0 && j == 0 {
        MapTile::Flagstone
      } else if i % 2 == 0 && j % 2 == 0 {
        MapTile::FloweryGrass
      } else {
        MapTile::Grass
      };
      map.insert(pos, Tile {
        _type,
        variant: (i + 10) as usize * (j + 10) as usize,
      });
    }
  }

  for (pos, tile) in map {
    commands.spawn(SpriteSheetBundle {
      texture_atlas: tile._type.atlas_handle(&atlases).atlas,
      transform: pos.transform(&tile._type),
      sprite: tile.texture_atlas_sprite(&atlases),
      ..Default::default()
    });
  }
}
