use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

use crate::{
    common::{LayerWorm, TileTree, WORM_SPAWN_CHANCE},
    gamedate_plugin::GameDateOneDayEvent,
    tile_worm_plugin::SpawnWormTileEvent,
};

use rand::Rng;

pub struct LogicWormPlugin;

impl Plugin for LogicWormPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_warm_random);
    }
}

fn spawn_warm_random(
    oneday_event_reader: EventReader<GameDateOneDayEvent>,
    q_tree: Query<&TilePos, With<TileTree>>,
    q_storage_worm: Query<&TileStorage, With<LayerWorm>>,
    mut spawn_worm_event_writer: EventWriter<SpawnWormTileEvent>,
) {
    if oneday_event_reader.is_empty() {
        return;
    }
    oneday_event_reader.clear();

    let t_storag_worm = q_storage_worm.single();

    let mut rand = rand::thread_rng();

    for tile_pos in q_tree.iter() {
        if t_storag_worm.get(&tile_pos).is_none() && rand.gen_bool(WORM_SPAWN_CHANCE) {
            spawn_worm_event_writer.send(SpawnWormTileEvent(tile_pos.x, tile_pos.y));
        }
    }
}
