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
    part:   VerticalPart,
  },
}

impl TileType for MapTile {
  fn size_and_center(&self) -> (Vec2, Vec2) {
    match self {
      MapTile::Grass | MapTile::FloweryGrass | MapTile::Flagstone => {
        (Vec2::splat(16.0), Vec2::ZERO)
      }
      MapTile::TallWall { .. } => (Vec2::splat(16.0), Vec2::ZERO),
    }
  }
  fn coords(&self) -> Vec<TileSheetCoords> {
    match self {
      MapTile::Grass => rect_range_with_x_flip(0, 0, 4, 4),
      MapTile::FloweryGrass => rect_range_with_x_flip(4, 0, 4, 4),
      MapTile::Flagstone => rect_range_with_x_flip(0, 4, 2, 3),
      MapTile::TallWall { corner, part } => match corner {
        Direction8::North => match part {
          VerticalPart::Top => vec![TileSheetCoords::new(2, 1)],
          VerticalPart::Bottom => vec![TileSheetCoords::new(2, 2)],
        },
        Direction8::NorthEast => match part {
          VerticalPart::Top => vec![TileSheetCoords::new(3, 1)],
          VerticalPart::Bottom => vec![TileSheetCoords::new(3, 2)],
        },
        Direction8::East => match part {
          VerticalPart::Top => vec![TileSheetCoords::new(3, 2)],
          VerticalPart::Bottom => vec![TileSheetCoords::new(2, 2)],
        },
        Direction8::SouthEast => match part {
          VerticalPart::Top => vec![TileSheetCoords::new(3, 3)],
          VerticalPart::Bottom => vec![TileSheetCoords::new(3, 4)],
        },
        Direction8::South => match part {
          VerticalPart::Top => vec![TileSheetCoords::new(2, 3)],
          VerticalPart::Bottom => vec![TileSheetCoords::new(2, 4)],
        },
        Direction8::SouthWest => match part {
          VerticalPart::Top => vec![TileSheetCoords::new(1, 3)],
          VerticalPart::Bottom => vec![TileSheetCoords::new(1, 4)],
        },
        Direction8::West => match part {
          VerticalPart::Top => vec![TileSheetCoords::new(1, 2)],
          VerticalPart::Bottom => vec![TileSheetCoords::new(2, 2)],
        },
        Direction8::NorthWest => match part {
          VerticalPart::Top => vec![TileSheetCoords::new(1, 1)],
          VerticalPart::Bottom => vec![TileSheetCoords::new(1, 2)],
        },
      },
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

  let mut wall_map = HashMap::new();
  wall_map.insert((3, 5), Direction8::SouthWest);
  wall_map.insert((5, 5), Direction8::South);
  wall_map.insert((7, 5), Direction8::SouthEast);
  wall_map.insert((3, 7), Direction8::West);
  wall_map.insert((7, 7), Direction8::East);
  wall_map.insert((3, 9), Direction8::NorthWest);
  wall_map.insert((5, 9), Direction8::North);
  wall_map.insert((7, 9), Direction8::NorthEast);

  for (pos, corner) in wall_map {
    map.insert(
      TilePosition::new(pos.0, pos.1, 1),
      Tile::new(MapTile::TallWall {
        corner,
        part: VerticalPart::Bottom,
      }),
    );
    map.insert(
      TilePosition::new(pos.0, pos.1 + 2, 2),
      Tile::new(MapTile::TallWall {
        corner,
        part: VerticalPart::Top,
      }),
    );
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
