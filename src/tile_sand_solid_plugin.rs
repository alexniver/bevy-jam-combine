use crate::common::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::*, TilemapBundle};

pub struct TileSandSolidPlugin;

impl Plugin for TileSandSolidPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSandSolidTileEvent>()
            .add_event::<DespawnSandSolidTileEvent>()
            .add_startup_system(setup_tilemap)
            .add_system(spawn_tile)
            .add_system(despawn_tile.before(spawn_tile));
    }
}

pub struct SpawnSandSolidTileEvent(pub u32, pub u32);
pub struct DespawnSandSolidTileEvent(pub u32, pub u32);

fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tileset-desertterminator.png");

    let tilemap_size = TilemapSize {
        x: MAP_SIZE,
        y: MAP_SIZE,
    };

    let tilemap_entity = commands.spawn().id();
    let tile_storage = TileStorage::empty(tilemap_size);

    let tile_size = TilemapTileSize {
        x: TILE_SIZE,
        y: TILE_SIZE,
    };

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                LAYER_Z_SAND_SOLID,
            ),
            ..Default::default()
        })
        .insert(LayerSandSolid);
}

fn spawn_tile(
    mut commands: Commands,
    mut q_storage: Query<(Entity, &mut TileStorage), With<LayerSandSolid>>,
    mut spawn_event_reader: EventReader<SpawnSandSolidTileEvent>,
) {
    if spawn_event_reader.is_empty() {
        return;
    }

    let (tilemap_entity, mut tile_storage) = q_storage.single_mut();
    for spawn_event in spawn_event_reader.iter() {
        let tile_pos = TilePos {
            x: spawn_event.0,
            y: spawn_event.1,
        };
        let tile_entity = commands
            .spawn()
            .insert_bundle(TileBundle {
                position: tile_pos,
                texture: TileTexture(3),
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            })
            .insert(TileSandSolid(SAND_SOLID_DAYS))
            .id();
        tile_storage.set(&tile_pos, Some(tile_entity));
    }
}

fn despawn_tile(
    mut commands: Commands,
    mut q_storage: Query<&mut TileStorage, With<LayerSandSolid>>,
    mut spawn_event_reader: EventReader<DespawnSandSolidTileEvent>,
) {
    if spawn_event_reader.is_empty() {
        return;
    }
    let mut tile_storage = q_storage.single_mut();
    for spawn_event in spawn_event_reader.iter() {
        let tile_pos = TilePos {
            x: spawn_event.0,
            y: spawn_event.1,
        };

        let tile_entity = tile_storage.get(&tile_pos);
        if let Some(tile_entity) = tile_entity {
            commands.entity(tile_entity).despawn_recursive();
            tile_storage.set(&tile_pos, None);
        }
    }
}
