use bevy::prelude::*;
use bevy_ecs_tilemap::{helpers, prelude::*};

use crate::common::*;

pub struct SolarPanelPlugin;

impl Plugin for SolarPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_solar_panel_tilemap);
    }
}

fn setup_solar_panel_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            transform: helpers::get_centered_transform_2d(&tilemap_size, &tile_size, 2.0),
            ..default()
        })
        .insert(LayerSolarPanel);
}
