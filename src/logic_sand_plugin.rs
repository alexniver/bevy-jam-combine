use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

use crate::{
    build_plugin::{BuildSolidEvent, BuildTreeEvent},
    common::*,
    tile_sand_plugin::DespawnSandTileEvent,
    tile_sand_solid_plugin::SpawnSandSolidTileEvent,
    tile_tree_little_plugin::SpawnTreeLittleTileEvent,
};

pub struct LogicSandPlugin;

impl Plugin for LogicSandPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(solid_event).add_system(plant_tree_event);
    }
}

fn solid_event(
    mut solid_event_reader: EventReader<BuildSolidEvent>,
    q_storage: Query<&TileStorage, With<LayerSand>>,
    q_sand: Query<(), With<TileSand>>,
    mut despawn_sand_event_writer: EventWriter<DespawnSandTileEvent>,
    mut spawn_sand_solid_event_writer: EventWriter<SpawnSandSolidTileEvent>,
) {
    if solid_event_reader.is_empty() {
        return;
    }

    let tile_storage = q_storage.single();

    for event in solid_event_reader.iter() {
        let tile_pos = TilePos {
            x: event.0,
            y: event.1,
        };
        let tile_entity = tile_storage.get(&tile_pos);
        if let Some(tile_entity) = tile_entity {
            if q_sand.contains(tile_entity) {
                despawn_sand_event_writer.send(DespawnSandTileEvent(tile_pos.x, tile_pos.y));
                spawn_sand_solid_event_writer.send(SpawnSandSolidTileEvent(tile_pos.x, tile_pos.y));
            }
        }
    }
}

fn plant_tree_event(
    mut tree_event_reader: EventReader<BuildTreeEvent>,
    q_storage_sand: Query<
        &TileStorage,
        (With<LayerSand>, Without<LayerSandSolid>, Without<LayerTree>),
    >,
    q_storage_sand_solid: Query<
        &TileStorage,
        (With<LayerSandSolid>, Without<LayerSand>, Without<LayerTree>),
    >,
    q_storage_tree: Query<
        &TileStorage,
        (With<LayerTree>, Without<LayerSand>, Without<LayerSandSolid>),
    >,
    mut spawn_tree_little_event_writer: EventWriter<SpawnTreeLittleTileEvent>,
) {
    if tree_event_reader.is_empty() {
        return;
    }

    let tile_storage_sand = q_storage_sand.single();
    let tile_storage_sand_solid = q_storage_sand_solid.single();
    let tile_storage_tree = q_storage_tree.single();

    for event in tree_event_reader.iter() {
        let tile_pos = TilePos {
            x: event.0,
            y: event.1,
        };

        if (tile_storage_sand.get(&tile_pos).is_some()
            || tile_storage_sand_solid.get(&tile_pos).is_some())
            && tile_storage_tree.get(&tile_pos).is_none()
        {
            spawn_tree_little_event_writer.send(SpawnTreeLittleTileEvent(tile_pos.x, tile_pos.y))
        }
    }
}
