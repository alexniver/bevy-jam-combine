use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers,
    prelude::{TilemapGridSize, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize},
    tiles::*,
    TilemapBundle,
};

use crate::{common::*, gamedate_plugin::GameDateOneDayEvent};

pub struct TreePlugin;
impl Plugin for TreePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<BuildTreeLittleEvent>()
            .add_startup_system(setup_tree_tilemap)
            .add_system(build_tree)
            .add_system(little_tree_grow)
            .add_system(sand_damage_little_tree);
    }
}

pub struct BuildTreeLittleEvent(pub u32, pub u32);

fn setup_tree_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tileset-desertterminator.png");
    let tilemap_size = TilemapSize {
        x: MAP_SIZE,
        y: MAP_SIZE,
    };

    // base
    let tilemap_entity = commands.spawn().id();

    let tile_storage = TileStorage::empty(tilemap_size);

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
            transform: helpers::get_centered_transform_2d(&tilemap_size, &tile_size, 1.0),
            ..default()
        })
        .insert(LayerTree);
}

fn build_tree(
    mut commands: Commands,
    mut build_tree_event: EventReader<BuildTreeLittleEvent>,

    // mut param_set: ParamSet<(
    //     Query<(Entity, &mut TileStorage), With<LayerTree>>,
    //     Query<&TileStorage, With<LayerSolarPanel>>,
    // )>,
    mut q_tree: Query<(Entity, &mut TileStorage), (With<LayerTree>, Without<LayerSolarPanel>)>,
    q_solar_panel: Query<&TileStorage, (With<LayerSolarPanel>, Without<LayerTree>)>,
) {
    if build_tree_event.is_empty() {
        return;
    }

    // let q_tree = param_set.p0();
    // let q_solar_panel = param_set.p1();

    let (tree_tilemap_entity, mut tree_tile_storage) = q_tree.single_mut();
    let solar_panel_tile_storage = q_solar_panel.single();

    for event in build_tree_event.iter() {
        let tile_pos = TilePos {
            x: event.0,
            y: event.1,
        };

        let tree_tile_some = tree_tile_storage.get(&tile_pos);
        let solar_panel_tile_some = solar_panel_tile_storage.get(&tile_pos);

        if tree_tile_some.is_none() && solar_panel_tile_some.is_none() {
            spawn_tree_little(
                &mut commands,
                &mut tree_tile_storage,
                tree_tilemap_entity,
                tile_pos.x,
                tile_pos.y,
            )
        }
    }
}

fn little_tree_grow(
    mut commands: Commands,
    oneday_event: EventReader<GameDateOneDayEvent>,
    mut q_storage: Query<(Entity, &mut TileStorage), With<LayerTree>>,
    mut q_little_tree: Query<(Entity, &mut TileTreeLittle, &TilePos)>,
) {
    if oneday_event.is_empty() {
        return;
    }
    oneday_event.clear();

    let (tree_tilemap_entity, mut tile_storage) = q_storage.single_mut();

    for (tile_entity, mut little_tree, tile_pos) in q_little_tree.iter_mut() {
        little_tree.0 -= 1;
        if little_tree.0 <= 0 {
            commands.entity(tile_entity).despawn_recursive();
            spawn_tree(
                &mut commands,
                &mut tile_storage,
                tree_tilemap_entity,
                tile_pos.x,
                tile_pos.y,
            )
        }
    }
}

fn sand_damage_little_tree(
    mut commands: Commands,
    oneday_event: EventReader<GameDateOneDayEvent>,
    mut q_base_storage: Query<&TileStorage, With<LayerSand>>,
    mut q_little_tree: Query<(Entity, &mut TileTreeLittle, &TilePos)>,
) {
    if oneday_event.is_empty() {
        return;
    }
    oneday_event.clear();

    for (tile_entity, mut little_tree, tile_pos) in q_little_tree.iter_mut() {
        little_tree.1 -= 1;
        if little_tree.1 <= 0 {
            commands.entity(tile_entity).despawn_recursive();
        }
    }
}

fn spawn_tree_little(
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
            texture: TileTexture(15),
            tilemap_id: TilemapId(tilemap_entity),
            ..default()
        })
        .insert(TileTreeLittle(5, 2))
        .id();

    tile_storage.set(&tile_pos, Some(tile_entity));
}

fn spawn_tree(
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
            texture: TileTexture(16),
            tilemap_id: TilemapId(tilemap_entity),
            ..default()
        })
        .insert(TileTree(100))
        .id();

    tile_storage.set(&tile_pos, Some(tile_entity));
}
