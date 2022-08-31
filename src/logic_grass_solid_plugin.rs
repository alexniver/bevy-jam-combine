use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{
    common::TileGrassSolid, gamedate_plugin::GameDateOneDayEvent,
    tile_grass_plugin::SpawnGrassTileEvent, tile_grass_solid_plugin::DespawnGrassSolidTileEvent,
};

pub struct LogicGrassSolidPlugin;

impl Plugin for LogicGrassSolidPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(lose_hp_daily);
    }
}

fn lose_hp_daily(
    oneday_event_reader: EventReader<GameDateOneDayEvent>,
    mut q: Query<(&mut TileGrassSolid, &TilePos)>,
    mut despawn_tile_solid_writer: EventWriter<DespawnGrassSolidTileEvent>,
    mut spawn_tile_grass_writer: EventWriter<SpawnGrassTileEvent>,
) {
    if oneday_event_reader.is_empty() {
        return;
    }
    oneday_event_reader.clear();

    for (mut tile_grass_solid, tile_pos) in q.iter_mut() {
        tile_grass_solid.0 -= 1;
        if tile_grass_solid.0 == 0 {
            despawn_tile_solid_writer.send(DespawnGrassSolidTileEvent(tile_pos.x, tile_pos.y));
            spawn_tile_grass_writer.send(SpawnGrassTileEvent(tile_pos.x, tile_pos.y));
        }
    }
}
