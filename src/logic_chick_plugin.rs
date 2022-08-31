use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

use crate::{
    build_plugin::BuildChickEvent,
    common::{LayerChick, LayerChicken, LockInWorm, TileChick, TileChicken, WormBeenLocked},
    gamedate_plugin::GameDateOneDayEvent,
    path_finding::next_grid,
    tile_chick_plugin::{DespawnChickTileEvent, SpawnChickTileEvent},
    tile_chicken_plugin::SpawnChickenTileEvent,
};

pub struct LogicChickPlugin;
impl Plugin for LogicChickPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(build_chick_event)
            .add_system(chick_grow_up)
            .add_system(chick_lock_worm_oneday)
            .add_system(chick_walk_to_worm);
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

fn chick_grow_up(
    oneday_event_reader: EventReader<GameDateOneDayEvent>,
    mut q_chick: Query<(&mut TileChick, &TilePos)>,
    mut despawn_chick_event_writer: EventWriter<DespawnChickTileEvent>,
    mut spawn_chicken_event_writer: EventWriter<SpawnChickenTileEvent>,
) {
    if oneday_event_reader.is_empty() {
        return;
    }
    oneday_event_reader.clear();

    for (mut tile_chick, tile_pos) in q_chick.iter_mut() {
        tile_chick.0 -= 1;
        if tile_chick.0 == 0 {
            despawn_chick_event_writer.send(DespawnChickTileEvent(tile_pos.x, tile_pos.y));
            spawn_chicken_event_writer.send(SpawnChickenTileEvent(tile_pos.x, tile_pos.y));
        }
    }
}

fn chick_lock_worm_oneday(
    mut commands: Commands,
    oneday_event: EventReader<GameDateOneDayEvent>,
    q_chick_no_lock: Query<(Entity, &TilePos), Without<LockInWorm>>,
    q_worm_not_been_locked: Query<(Entity, &TilePos), Without<WormBeenLocked>>,
) {
    if oneday_event.is_empty() {
        return;
    }
    oneday_event.clear();

    if q_chick_no_lock.is_empty() || q_worm_not_been_locked.is_empty() {
        return;
    }

    for (chick_entity, chick_pos) in q_chick_no_lock.iter() {
        let mut min_distance = f32::MAX;
        let chick_pos = Vec2::new(chick_pos.x as f32, chick_pos.y as f32);
        let mut selected_worm_entity = None;
        let mut selected_worm_pos = default();
        for (worm_entity, worm_pos) in q_worm_not_been_locked.iter() {
            let worm_pos = Vec2::new(worm_pos.x as f32, worm_pos.y as f32);
            let len = (chick_pos - worm_pos).length();
            if min_distance > len {
                min_distance = len;
                selected_worm_entity = Some(worm_entity);
                selected_worm_pos = worm_pos;
            }
        }

        if let Some(selected_worm_entity) = selected_worm_entity {
            commands.entity(selected_worm_entity).insert(WormBeenLocked);
            commands.entity(chick_entity).insert(LockInWorm(
                selected_worm_pos.x as u32,
                selected_worm_pos.y as u32,
            ));
        }
    }
}

fn chick_walk_to_worm(
    oneday_event: EventReader<GameDateOneDayEvent>,
    q_chick: Query<(&TilePos, &LockInWorm)>,
    q_stroage_chick: Query<&TileStorage, (With<TileChick>, Without<TileChicken>)>,
    q_storage_chicken: Query<&TileStorage, (With<TileChicken>, Without<TileChick>)>,
) {
    if oneday_event.is_empty() {
        return;
    }
    oneday_event.clear();

    let t_storage_chick = q_stroage_chick.single();
    let t_storage_chicken = q_storage_chicken.single();

    for (chick_pos, worm_pos) in q_chick.iter() {
        for next_pos in next_grid(
            Vec2::new(chick_pos.x as f32, chick_pos.y as f32),
            Vec2::new(worm_pos.0 as f32, worm_pos.1 as f32),
        )
        .iter()
        {
            let next_tile_pos = TilePos::new(next_pos.x as u32, next_pos.y as u32);
            if t_storage_chick.get(&next_tile_pos).is_none()
                && t_storage_chicken.get(&next_tile_pos).is_none()
            {}
        }
    }
}
