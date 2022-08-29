use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

use crate::{
    common::{LayerSand, TileTreeLittle, MAP_SIZE},
    gamedate_plugin::GameDateOneDayEvent,
    tile_grass_plugin::SpawnGrassTileEvent,
    tile_sand_plugin::DespawnSandTileEvent,
    tile_sand_solid_plugin::DespawnSandSolidTileEvent,
    tile_tree_little_plugin::DespawnTreeLittleTileEvent,
    tile_tree_plugin::SpawnTreeTileEvent,
};

pub struct LogicTreeLittlePlugin;

impl Plugin for LogicTreeLittlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tree_grow).add_system(damage_by_sand);
    }
}

fn tree_grow(
    mut q_little_tree: Query<(&mut TileTreeLittle, &TilePos)>,
    oneday_event: EventReader<GameDateOneDayEvent>,
    mut despawn_tree_little_event_writer: EventWriter<DespawnTreeLittleTileEvent>,
    mut spawn_tree_event_writer: EventWriter<SpawnTreeTileEvent>,
    mut despawn_sand_event_writer: EventWriter<DespawnSandTileEvent>,
    mut despawn_sand_solid_event_writer: EventWriter<DespawnSandSolidTileEvent>,
    mut spawn_grass_evetn_writer: EventWriter<SpawnGrassTileEvent>,
) {
    if oneday_event.is_empty() {
        return;
    }
    oneday_event.clear();

    for (mut tree_little, tile_pos) in q_little_tree.iter_mut() {
        tree_little.0 -= 1;
        if tree_little.0 == 0 {
            despawn_tree_little_event_writer
                .send(DespawnTreeLittleTileEvent(tile_pos.x, tile_pos.y));
            spawn_tree_event_writer.send(SpawnTreeTileEvent(tile_pos.x, tile_pos.y));
            despawn_sand_event_writer.send(DespawnSandTileEvent(tile_pos.x, tile_pos.y));
            despawn_sand_solid_event_writer.send(DespawnSandSolidTileEvent(tile_pos.x, tile_pos.y));
            spawn_grass_evetn_writer.send(SpawnGrassTileEvent(tile_pos.x, tile_pos.y));
        }
    }
}

fn damage_by_sand(
    oneday_event_reader: EventReader<GameDateOneDayEvent>,
    mut q_grass: Query<(&mut TileTreeLittle, &TilePos)>,
    q_storage_sand: Query<&TileStorage, With<LayerSand>>,
    mut despawn_tree_little_event_writer: EventWriter<DespawnTreeLittleTileEvent>,
) {
    if oneday_event_reader.is_empty() {
        return;
    }
    oneday_event_reader.clear();

    let t_storage_sand = q_storage_sand.single();
    for (mut tile_tree_little, tile_pos) in q_grass.iter_mut() {
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
                tile_tree_little.1 -= 1;
                if tile_tree_little.1 <= 0 {
                    despawn_tree_little_event_writer
                        .send(DespawnTreeLittleTileEvent(tile_pos.x, tile_pos.y));
                    break;
                }
            }
        }
    }
}
