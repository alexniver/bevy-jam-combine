/*use crate::{common::*, gamedate_plugin::GameDateOneDayEvent};
use bevy::prelude::*;
use bevy_ecs_tilemap::{helpers, prelude::*, tiles::*, TilemapBundle};
use rand::Rng;

pub struct BasetailPlugin;

impl Plugin for BasetailPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildSolidEvent>()
            .add_startup_system(setup_base_tilemap)
            // .add_system(setup_sand_nearby_grass);
            // .add_startup_system(setup_sand_nearby_grass.after(setup_base_tilemap));
            // .add_startup_system_to_stage(StartupStage::PostStartup, setup_sand_nearby_grass)
            .add_system(sand_damage_grass)
            .add_system(sand_grass_solid)
            .add_system(solid_daily_subhp);
    }
}

pub struct BuildSolidEvent(pub u32, pub u32);

fn setup_base_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tileset-desertterminator.png");
    let tilemap_size = TilemapSize {
        x: MAP_SIZE,
        y: MAP_SIZE,
    };

    // base
    let tilemap_entity = commands.spawn().id();

    let mut tile_storage = TileStorage::empty(tilemap_size);
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
                spawn_tile_sand(&mut commands, &mut tile_storage, tilemap_entity, x, y)
            } else {
                spawn_tile_grass(&mut commands, &mut tile_storage, tilemap_entity, x, y)
            }
        }
    }

    let tile_size = TilemapTileSize {
        x: TILE_SIZE,
        y: TILE_SIZE,
    };

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle.clone()),
            tile_size,
            transform: helpers::get_centered_transform_2d(&tilemap_size, &tile_size, 0.0),
            ..default()
        })
        .insert(LayerBase);
}

fn sand_damage_grass(
    mut commands: Commands,
    mut q_storage: Query<(Entity, &mut TileStorage), With<LayerBase>>,
    mut q_grass: Query<(Entity, &TilePos, &mut TileGrass)>,
    q_sand: Query<(), With<TileSand>>,
    one_day_event: EventReader<GameDateOneDayEvent>,
) {
    if one_day_event.is_empty() {
        return;
    }

    let (tilemap_entity, mut tile_storage) = q_storage.single_mut();

    for (grass_tile_entity, grass_tile_pos, mut tile_grass) in q_grass.iter_mut() {
        let mut neighbour_pos_arr = Vec::new();
        let (x, y) = (grass_tile_pos.x as u32, grass_tile_pos.y as u32);
        if x > 0 {
            neighbour_pos_arr.push(TilePos { x: x - 1, y });
        }
        if x < MAP_SIZE - 1 {
            neighbour_pos_arr.push(TilePos { x: x + 1, y });
        }
        if y > 0 {
            neighbour_pos_arr.push(TilePos { x, y: y - 1 });
        }
        if y < MAP_SIZE - 1 {
            neighbour_pos_arr.push(TilePos { x, y: y + 1 });
        }

        for neighbour_pos in neighbour_pos_arr {
            if let Some(tile_entity) = tile_storage.get(&neighbour_pos) {
                if q_sand.contains(tile_entity) {
                    tile_grass.0 -= 30;
                    if tile_grass.0 <= 0 {
                        commands.entity(grass_tile_entity).despawn_recursive();
                        spawn_tile_sand(&mut commands, &mut tile_storage, tilemap_entity, x, y);
                        break;
                    }
                }
            }
        }
    }

    one_day_event.clear();
}

fn spawn_tile_grass(
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    tilemap_entity: Entity,
    x: u32,
    y: u32,
) {
    let tile_pos = TilePos { x, y };
    let tile_entity = commands
        .spawn()
        .insert_bundle(TileBundle {
            position: tile_pos,
            texture: TileTexture(0),
            tilemap_id: TilemapId(tilemap_entity),
            ..default()
        })
        .insert(TileGrass(100))
        .id();

    tile_storage.set(&tile_pos, Some(tile_entity));
}

fn spawn_tile_grass_solid(
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    tilemap_entity: Entity,
    x: u32,
    y: u32,
) {
    let tile_pos = TilePos { x, y };
    let tile_entity = commands
        .spawn()
        .insert_bundle(TileBundle {
            position: tile_pos,
            texture: TileTexture(1),
            tilemap_id: TilemapId(tilemap_entity),
            ..default()
        })
        .insert(TileGrassSolid(30 * 6))
        .id();

    tile_storage.set(&tile_pos, Some(tile_entity));
}

fn spawn_tile_sand(
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    tilemap_entity: Entity,
    x: u32,
    y: u32,
) {
    let tile_pos = TilePos { x, y };
    let tile_entity = commands
        .spawn()
        .insert_bundle(TileBundle {
            position: tile_pos,
            texture: TileTexture(2),
            tilemap_id: TilemapId(tilemap_entity),
            ..default()
        })
        .insert(TileSand)
        .id();

    tile_storage.set(&tile_pos, Some(tile_entity));
}

fn spawn_tile_sand_solid(
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    tilemap_entity: Entity,
    x: u32,
    y: u32,
) {
    let tile_pos = TilePos { x, y };
    let tile_entity = commands
        .spawn()
        .insert_bundle(TileBundle {
            position: tile_pos,
            texture: TileTexture(3),
            tilemap_id: TilemapId(tilemap_entity),
            ..default()
        })
        .insert(TileSandSolid(30 * 6))
        .id();

    tile_storage.set(&tile_pos, Some(tile_entity));
}

// make sand and grass solid
fn sand_grass_solid(
    mut commands: Commands,
    mut q_storage: Query<(Entity, &mut TileStorage), With<LayerBase>>,
    q_grass: Query<(), With<TileGrass>>,
    q_sand: Query<(), With<TileSand>>,
    mut solid_event: EventReader<BuildSolidEvent>,
) {
    let (tilemap_entity, mut tile_storage) = q_storage.single_mut();
    for e in solid_event.iter() {
        info!("soild event: {} {}", e.0, e.1);
        let tile_pos = TilePos { x: e.0, y: e.1 };
        let tile_entity = tile_storage.get(&tile_pos);
        if let Some(tile_entity) = tile_entity {
            if q_grass.contains(tile_entity) {
                commands.entity(tile_entity).despawn_recursive();
                spawn_tile_grass_solid(
                    &mut commands,
                    &mut tile_storage,
                    tilemap_entity,
                    tile_pos.x,
                    tile_pos.y,
                );
            } else if q_sand.contains(tile_entity) {
                commands.entity(tile_entity).despawn_recursive();
                spawn_tile_sand_solid(
                    &mut commands,
                    &mut tile_storage,
                    tilemap_entity,
                    tile_pos.x,
                    tile_pos.y,
                );
            }
        }
    }
}

fn solid_daily_subhp(
    mut commands: Commands,
    mut q_storage: Query<(Entity, &mut TileStorage), With<LayerBase>>,
    oneday_event: EventReader<GameDateOneDayEvent>,
    mut q_grass_solid: Query<(Entity, &mut TileGrassSolid, &TilePos)>,
    mut q_sand_solid: Query<(Entity, &mut TileSandSolid, &TilePos)>,
) {
    if oneday_event.is_empty() {
        return;
    }
    oneday_event.clear();

    let (tilemap_entity, mut tile_storage) = q_storage.single_mut();

    for (tile_entity, mut grass_solid, tile_pos) in q_grass_solid.iter_mut() {
        grass_solid.0 -= 1;
        if grass_solid.0 <= 0 {
            commands.entity(tile_entity).despawn_recursive();
            spawn_tile_grass(
                &mut commands,
                &mut tile_storage,
                tilemap_entity,
                tile_pos.x,
                tile_pos.y,
            );
        }
    }

    for (tile_entity, mut sand_solid, tile_pos) in q_sand_solid.iter_mut() {
        sand_solid.0 -= 1;
        if sand_solid.0 <= 0 {
            commands.entity(tile_entity).despawn_recursive();
            spawn_tile_sand(
                &mut commands,
                &mut tile_storage,
                tilemap_entity,
                tile_pos.x,
                tile_pos.y,
            );
        }
    }
}
*/
