use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

use crate::{
    build_plugin::BuildChickEvent,
    common::{LayerChick, LayerChicken},
    tile_chick_plugin::SpawnChickTileEvent,
};

pub struct LogicChickPlugin;
impl Plugin for LogicChickPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(build_chick_event);
    }
}

fn build_chick_event(
    mut build_chick_event: EventReader<BuildChickEvent>,
    q_storage_chick: Query<&TileStorage, (With<LayerChick>, Without<LayerChicken>)>,
    q_storage_chicken: Query<&TileStorage, (With<LayerChicken>, Without<LayerChick>)>,
    mut spawn_chick_event_writer: EventWriter<SpawnChickTileEvent>,
) {
    if build_chick_event.is_empty() {
        return;
    }

    let t_storage_chick = q_storage_chick.single();
    let t_storage_chicken = q_storage_chicken.single();

    for event in build_chick_event.iter() {
        let tile_pos = TilePos {
            x: event.0,
            y: event.1,
        };
        if t_storage_chick.get(&tile_pos).is_none() && t_storage_chicken.get(&tile_pos).is_none() {
            spawn_chick_event_writer.send(SpawnChickTileEvent(tile_pos.x, tile_pos.y));
        }
    }
}
