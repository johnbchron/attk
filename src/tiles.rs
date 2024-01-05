use bevy::{prelude::*, utils::HashMap};

enum Direction {
  North,
  East,
  South,
  West,
}

enum TileType {
  Grass,
  FloweryGrass,
  Flagstone,
  TallWall(Direction),
}

struct Tile {
  _type:   TileType,
  variant: u8,
}

impl Tile {
  fn passable(&self) -> bool {
    match self._type {
      TileType::Grass => true,
      TileType::FloweryGrass => true,
      TileType::Flagstone => true,
      TileType::TallWall(_) => false,
    }
  }
  fn atlas_handle(&self, atlases: &TileAtlases) -> Handle<TextureAtlas> {
    match self._type {
      TileType::Grass | TileType::FloweryGrass | TileType::Flagstone => {
        atlases.grass.clone()
      }
      TileType::TallWall(_) => atlases.wall.clone(),
    }
  }
  fn index(&self) -> usize {
    match &self._type {
      TileType::Grass => 2,
      TileType::FloweryGrass => 29,
      TileType::Flagstone => 128,
      TileType::TallWall(direction) => match direction {
        Direction::North => 19,
        Direction::East => 16,
        Direction::South => 28,
        Direction::West => 41,
      },
    }
  }
  fn size(&self) -> Vec2 {
    match self._type {
      TileType::Grass | TileType::FloweryGrass | TileType::Flagstone => {
        Vec2::splat(16.0)
      }
      TileType::TallWall(_) => Vec2::splat(32.0),
    }
  }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TilePosition(i64, i64);

impl TilePosition {
  fn world_position(&self) -> Vec3 {
    Vec3::new(self.0 as f32, self.1 as f32, 0.0)
  }
}

#[derive(Resource, Clone)]
struct TileAtlases {
  grass: Handle<TextureAtlas>,
  wall:  Handle<TextureAtlas>,
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
  fn build(&self, app: &mut App) { app.add_systems(Startup, setup); }
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let grass_texture_handle = asset_server.load("textures/tiles/grass.png");
  let grass_atlas = TextureAtlas::from_grid(
    grass_texture_handle,
    Vec2::new(16.0, 16.0),
    16,
    16,
    None,
    None,
  );
  let grass_atlas_handle = texture_atlases.add(grass_atlas);

  let wall_texture_handle = asset_server.load("textures/tiles/wall.png");
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
  commands.insert_resource(atlases.clone());

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
      map.insert(TilePosition(i, j), Tile {
        _type:   TileType::Grass,
        variant: 0,
      });
    }
  }

  for (pos, tile) in map {
    commands.spawn(SpriteSheetBundle {
      texture_atlas: tile.atlas_handle(&atlases),
      transform: Transform::from_scale(tile.size().extend(1.0).recip())
        .with_translation(pos.world_position()),
      sprite: TextureAtlasSprite::new(tile.index()),
      ..Default::default()
    });
  }
}
