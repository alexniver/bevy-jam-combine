use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{
    common::TileSandSolid, gamedate_plugin::GameDateOneDayEvent,
    tile_sand_plugin::SpawnSandTileEvent, tile_sand_solid_plugin::DespawnSandSolidTileEvent,
};

pub struct LogicSandSolidPlugin;

impl Plugin for LogicSandSolidPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(solid_sand_lose_hp_daily);
    }
}

fn solid_sand_lose_hp_daily(
    oneday_event_reader: EventReader<GameDateOneDayEvent>,
    mut q: Query<(&mut TileSandSolid, &TilePos)>,
    mut despawn_sand_solid_writer: EventWriter<DespawnSandSolidTileEvent>,
    mut spawn_sand_writer: EventWriter<SpawnSandTileEvent>,
) {
    if oneday_event_reader.is_empty() {
        return;
    }
    oneday_event_reader.clear();

    for (mut tile_sand_solid, tile_pos) in q.iter_mut() {
        tile_sand_solid.0 -= 1;
        if tile_sand_solid.0 == 0 {
            despawn_sand_solid_writer.send(DespawnSandSolidTileEvent(tile_pos.x, tile_pos.y));
            spawn_sand_writer.send(SpawnSandTileEvent(tile_pos.x, tile_pos.y));
        }
    }
}
