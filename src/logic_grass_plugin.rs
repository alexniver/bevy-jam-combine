use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::*;

use crate::{
    build_plugin::{BuildSolidEvent, BuildTreeEvent},
    common::*,
    gamedate_plugin::GameDateOneDayEvent,
    tile_grass_plugin::DespawnGrassTileEvent,
    tile_grass_solid_plugin::SpawnGrassSolidTileEvent,
    tile_sand_plugin::SpawnSandTileEvent,
    tile_tree_little_plugin::SpawnTreeLittleTileEvent,
};

pub struct LogicGrassPlugin;
impl Plugin for LogicGrassPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(solid_event)
            .add_system(plant_tree_event)
            .add_system(damage_by_sand);
    }
}

fn solid_event(
    mut solid_event_reader: EventReader<BuildSolidEvent>,
    q_storage: Query<&TileStorage, With<LayerGrass>>,
    q_sand: Query<(), With<TileGrass>>,
    mut despawn_sand_event_writer: EventWriter<DespawnGrassTileEvent>,
    mut spawn_sand_solid_event_writer: EventWriter<SpawnGrassSolidTileEvent>,
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
                despawn_sand_event_writer.send(DespawnGrassTileEvent(tile_pos.x, tile_pos.y));
                spawn_sand_solid_event_writer
                    .send(SpawnGrassSolidTileEvent(tile_pos.x, tile_pos.y));
            }
        }
    }
}

fn plant_tree_event(
    mut tree_event_reader: EventReader<BuildTreeEvent>,
    q_storage_grass: Query<
        &TileStorage,
        (
            With<LayerGrass>,
            Without<LayerGrassSolid>,
            Without<LayerTree>,
        ),
    >,
    q_storage_grass_solid: Query<
        &TileStorage,
        (
            With<LayerGrassSolid>,
            Without<LayerGrass>,
            Without<LayerTree>,
        ),
    >,
    q_storage_tree: Query<
        &TileStorage,
        (
            With<LayerTree>,
            Without<LayerGrass>,
            Without<LayerSandSolid>,
        ),
    >,
    mut spawn_tree_little_event_writer: EventWriter<SpawnTreeLittleTileEvent>,
) {
    if tree_event_reader.is_empty() {
        return;
    }

    let tile_storage_grass = q_storage_grass.single();
    let tile_storage_grass_solid = q_storage_grass_solid.single();
    let tile_storage_tree = q_storage_tree.single();

    for event in tree_event_reader.iter() {
        let tile_pos = TilePos {
            x: event.0,
            y: event.1,
        };

        if (tile_storage_grass.get(&tile_pos).is_some()
            || tile_storage_grass_solid.get(&tile_pos).is_some())
            && tile_storage_tree.get(&tile_pos).is_none()
        {
            spawn_tree_little_event_writer.send(SpawnTreeLittleTileEvent(tile_pos.x, tile_pos.y))
        }
    }
}

fn damage_by_sand(
    oneday_event_reader: EventReader<GameDateOneDayEvent>,
    mut q_grass: Query<(&mut TileGrass, &TilePos)>,
    q_storage_sand: Query<&TileStorage, (With<LayerSand>, Without<LayerTree>)>,
    q_storage_tree: Query<&TileStorage, (With<LayerTree>, Without<LayerSand>)>,
    mut despawn_grass_event_writer: EventWriter<DespawnGrassTileEvent>,
    mut spawn_sand_event_writer: EventWriter<SpawnSandTileEvent>,
) {
    if oneday_event_reader.is_empty() {
        return;
    }
    oneday_event_reader.clear();

    let t_storage_sand = q_storage_sand.single();
    let t_storage_tree = q_storage_tree.single();
    for (mut tile_grass, tile_pos) in q_grass.iter_mut() {
        if t_storage_tree.get(&tile_pos).is_none() {
            let mut neighbour_pos_arr = vec![];
            if tile_pos.x > 0 {
                neighbour_pos_arr.push(TilePos {
                    x: tile_pos.x - 1,
                    y: tile_pos.y,
                });
            }

            if tile_pos.x < MAP_SIZE - 1 {
                neighbour_pos_arr.push(TilePos {
                    x: tile_pos.x + 1,
                    y: tile_pos.y,
                });
            }

            if tile_pos.y > 0 {
                neighbour_pos_arr.push(TilePos {
                    x: tile_pos.x,
                    y: tile_pos.y - 1,
                });
            }
            if tile_pos.y < MAP_SIZE - 1 {
                neighbour_pos_arr.push(TilePos {
                    x: tile_pos.x,
                    y: tile_pos.y + 1,
                });
            }

            for n_tile_pos in neighbour_pos_arr {
                if t_storage_sand.get(&n_tile_pos).is_some() {
                    tile_grass.0 -= 30;
                    if tile_grass.0 <= 0 {
                        despawn_grass_event_writer
                            .send(DespawnGrassTileEvent(tile_pos.x, tile_pos.y));
                        spawn_sand_event_writer.send(SpawnSandTileEvent(tile_pos.x, tile_pos.y));
                        break;
                    }
                }
            }
        }
    }
}
