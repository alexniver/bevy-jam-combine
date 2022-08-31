use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

use crate::{
    common::{LayerWorm, TileTree, TREE_HP},
    gamedate_plugin::GameDateOneDayEvent,
    tile_tree_plugin::DespawnTreeTileEvent,
    tile_worm_plugin::DespawnWormTileEvent,
};

pub struct LogicTreePlugin;
impl Plugin for LogicTreePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(worm_hurt_tree).add_system(sold_event);
    }
}

fn worm_hurt_tree(
    oneday_event: EventReader<GameDateOneDayEvent>,
    mut q_tree: Query<(&mut TileTree, &TilePos)>,
    q_storage_worm: Query<&TileStorage, With<LayerWorm>>,
    mut despawn_tree_event_writer: EventWriter<DespawnTreeTileEvent>,
    mut despawn_worm_event_writer: EventWriter<DespawnWormTileEvent>,
) {
    if oneday_event.is_empty() {
        return;
    }
    oneday_event.clear();

    let tile_storage_worm = q_storage_worm.single();

    for (mut tile_tree, tile_pos) in q_tree.iter_mut() {
        if tile_storage_worm.get(&tile_pos).is_some() {
            tile_tree.0 -= TREE_HP / 5;
            if tile_tree.0 <= 0 {
                despawn_tree_event_writer.send(DespawnTreeTileEvent(tile_pos.x, tile_pos.y));
                despawn_worm_event_writer.send(DespawnWormTileEvent(tile_pos.x, tile_pos.y));
            }
        }
    }
}

fn sold_event() {}
