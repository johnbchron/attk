use bevy::{prelude::*, utils::HashMap};

use crate::tile::{
  rect_range, Direction8, TextureAtlasWithGrid, Tile, TileAtlases,
  TilePosition, TileType, VerticalPart,
};

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
  fn size(&self) -> Vec2 { Vec2::splat(32.0) }
  fn coords(&self) -> Vec<(usize, usize)> {
    match self {
      MapTile::Grass => rect_range(0, 0, 4, 4),
      MapTile::FloweryGrass => rect_range(4, 0, 4, 4),
      MapTile::Flagstone => rect_range(0, 4, 2, 3),
      MapTile::TallWall { corner, height } => {
        // let (x, y) = match corner {
        //   Direction8::North => (0, 0),
        //   Direction8::NorthEast => (1, 0),
        //   Direction8::East => (2, 0),
        //   Direction8::SouthEast => (3, 0),
        //   Direction8::South => (0, 1),
        //   Direction8::SouthWest => (1, 1),
        //   Direction8::West => (2, 1),
        //   Direction8::NorthWest => (3, 1),
        // };
        // let y = match height {
        //   VerticalPart::Top => y,
        //   VerticalPart::Bottom => y + 2,
        // };
        // vec![(x, y)]
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
        x:     i,
        y:     j,
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
      sprite: TextureAtlasSprite::new(tile.index(&atlases)),
      ..Default::default()
    });
  }
}
