use crate::{
    common::MAP_SIZE, tile_grass_plugin::SpawnGrassTileEvent, tile_sand_plugin::SpawnSandTileEvent,
};
use bevy::prelude::*;
use rand::Rng;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(fill_map);
    }
}

fn fill_map(
    mut spawn_sand_writer: EventWriter<SpawnSandTileEvent>,
    mut spawn_grass_writer: EventWriter<SpawnGrassTileEvent>,
) {
    let mut rng = rand::thread_rng();
    let bias_len = 20;
    let min_bias = MAP_SIZE / 2 - bias_len / 2;
    let max_bias = MAP_SIZE / 2 + bias_len / 2;

    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            let mut bias = 0;
            if x > min_bias && x < max_bias {
                bias = rng.gen_range(0..bias_len);
            }
            if x < MAP_SIZE / 2 + bias {
                spawn_sand_writer.send(SpawnSandTileEvent(x, y));
            } else {
                spawn_grass_writer.send(SpawnGrassTileEvent(x, y));
            }
        }
    }
}
